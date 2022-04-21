package de.jadr;

import java.lang.invoke.SwitchPoint;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Locale;
import java.util.UUID;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import org.json.JSONObject;

import de.jadr.chess.ChessBoard;
import de.jadr.chess.ChessFigure;
import de.jadr.chess.Move;
import de.jadr.chess.VisualChessBoard;
import de.jadr.chess.ChessFigure.Type;
import de.jadr.chess.Move.Flag;
import de.jadr.chess.Move.FlagData;
import de.jadr.chess.KI.AsyncChessBruteForcer;
import de.jadr.chess.KI.ChessBruteForcer;
import de.jadr.chess.KI.ChessBruteForcer.KiMoveResult;
import de.jadr.utils.JSONBuilder;

public class ChessLobby {

	public static enum SystemMessageType {
		INFO, ERROR, WARNING, SUCCESS
	}

	public static enum PlayerStatus {
		WHITE, BLACK, SPECTATOR, NULL
	}

	private CustomWebSocketClient white;
	private final UUID lobbyUUID;

	private CustomWebSocketClient black = null;

	private ChessBoard board = new ChessBoard();
	private AsyncChessBruteForcer ki = new AsyncChessBruteForcer(board, false, 4);

	private static final ExecutorService LOBBY_EXECUTOR = Executors.newCachedThreadPool();

	private ArrayList<CustomWebSocketClient> spectators = new ArrayList<CustomWebSocketClient>();

	private VisualChessBoard vcb = null;

	public ChessLobby(UUID uuid, CustomWebSocketClient white, ChessWebSocketServer cwss) {
		System.out.println("lobby created");
		ChessLobby t = this;
		this.white = white;
		this.lobbyUUID = uuid;
		white.addOnClose(() -> {
			t.white = null;
			broadcast(new JSONBuilder().put("type", "hostleft").toString());
			cwss.lobbys.remove(lobbyUUID);
			if (vcb != null) {
				vcb.stop();
				vcb.getWindow().getJFrame().dispose();
			}

		});
		sendBoardToPlayer(white);
//		vcb = (VisualChessBoard) new VisualChessBoard("Game" + lobbyUUID, board);
//		
//		vcb.start();
	}

	private void sendMove(Move m) {
		JSONBuilder bs = new JSONBuilder().put("start", m.getStart()).put("end", m.getEnd()).put("type", "move");
		if (m.getFlagData() != null) {
			FlagData fd = m.getFlagData();
			bs.put("flag", new JSONBuilder().put("flag", fd.getFlag().ordinal()).put("data", fd.getData()).build());
		}
		broadcast(bs.toString());
	}

	public String formatStr(float val) {
		return String.format(Locale.CANADA, "%,.2f", val).split("\\.")[0].replaceAll(",", ".");
	}

	public void doMove(JSONObject moveObj, CustomWebSocketClient cws) {
		
		if(cws.equals(black)) {
			if(board.isWhitesTurn())return;
		}else if(cws.equals(white)) {
			if(!board.isWhitesTurn())return;
		}else {
			return;
		}
		
		JSONObject bs = new JSONBuilder(moveObj).put("type", "move").build();
		int start = bs.getInt("start");
		int end = bs.getInt("end");

		ChessFigure executor = board.getField(start).getFigure();
		if (executor == null)
			return;

		Move move = new Move(start, end, executor);
		if (moveObj.has("flag")) {
			JSONObject flagObj = moveObj.getJSONObject("flag");
			int flag = flagObj.getInt("flag");
			Object data = null;
			if (flagObj.has("data")) {
				data = flagObj.get("data");
			}

			move.setFlagData(new FlagData(Flag.values()[flag], data));

		}
		LOBBY_EXECUTOR.execute(() -> {
			if (!board.isMoveLegit(move)) {
				System.out.println(board.getField(63).getFigure().alreadyMoved());
				sendSystemMessage("Unable to execute move", SystemMessageType.ERROR, cws);
				return;
			}
			board.toggleTurn();

			executor.doMove(move);
			sendMove(move);
			broadcast(new JSONBuilder().put("type", "kiinfo")
					.put("turndata","")
					.put("sitestrenght", getSiteEvaluation()).put("FEN", board.generateFen()).toString());
			//doKiResponse();
		});
		
	}
	
	private String getSiteEvaluation() {
		int worth = ki.evalSite(board);
		String res = (worth<0?"Black":"White") + " " + (worth<0?worth*-1:worth);
		return res;
	}

	private void doKiResponse() {
		long now = System.currentTimeMillis();
		de.jadr.chess.KI.AsyncChessBruteForcer.KiMoveResult result = ki.doTurn();
		broadcast(new JSONBuilder().put("type", "kiinfo")
				.put("turndata",
						"Evaluated " + formatStr(result.getCalculations()) + " possible outcomes! ("
								+ (System.currentTimeMillis() - now) + "ms)")
				.put("sitestrenght", getSiteEvaluation()).put("FEN", board.generateFen()).toString());
		if (result.getMove() == null) {
			sendSystemMessage("[AI] GG wp", SystemMessageType.INFO);
		} else {
			board.toggleTurn();
			sendMove(result.getMove());
		}
		
	}

	private PlayerStatus getStatus(CustomWebSocketClient cwsc) {
		if (cwsc.equals(white))
			return PlayerStatus.WHITE;
		if (cwsc.equals(black))
			return PlayerStatus.BLACK;
		for (CustomWebSocketClient cwscs : spectators) {
			if (cwscs.equals(cwsc))
				return PlayerStatus.SPECTATOR;
		}
		return PlayerStatus.NULL;
	}

	public void sendMessage(JSONObject jo, CustomWebSocketClient cwsc) {
		JSONObject msgobj = jo.getJSONArray("args").getJSONObject(0);
		String msg = msgobj.getString("msg");
		if (msg.length() == 0)
			return;
		msgobj.put("msg", "[" + getStatus(cwsc).toString().toLowerCase() + "]: " + msg);
		if (msgobj.has("systemmessage"))
			return;
		broadcast(jo.toString());
	}

	public void sendSystemMessage(String message, SystemMessageType smt) {
		JSONBuilder jo = new JSONBuilder().put("type", "chatmessage");
		jo.put("args",
				Arrays.asList(new JSONBuilder().put("msg", message).put("systemmessage", smt.toString()).build()));
		broadcast(jo.toString());
	}
	
	public void sendSystemMessage(String message, SystemMessageType smt, CustomWebSocketClient socket) {
		JSONBuilder jo = new JSONBuilder().put("type", "chatmessage");
		jo.put("args",
				Arrays.asList(new JSONBuilder().put("msg", message).put("systemmessage", smt.toString()).build()));
		socket.getWebSocket().send(jo.toString());
	}

	private void broadcast(String str) {
		try {
			if (black != null)
				black.getWebSocket().send(str);
		} catch (Exception e) {
		}
		try {
			if (white != null)
				white.getWebSocket().send(str);
		} catch (Exception e) {
		}
		spectators.forEach(s -> {
			try {
				s.getWebSocket().send(str);
			} catch (Exception e) {
			}
		});
	}

	public void sendBoardToPlayer(CustomWebSocketClient cwsc) {
		
		cwsc.getWebSocket().send(new JSONBuilder().put("type", "fenMap").put("fen", board.generateFen()).toString());
	}

	public boolean join(CustomWebSocketClient player) {
		JSONBuilder jb = new JSONBuilder().put("type", "lobbyJoined");
		jb.put("lobbyid", this.lobbyUUID.toString());
		if (this.black == null) {
			this.black = player;
			jb.put("status", "black");
		} else {
			spectators.add(player);
			jb.put("status", "spectator");
		}
		player.addOnClose(() -> {
			sendSystemMessage(jb.build().get("status") + " left the lobby", SystemMessageType.ERROR);
		});
		player.getWebSocket().send(jb.toString());
		sendBoardToPlayer(player);
		sendSystemMessage(jb.build().getString("status") + " joined the lobby", SystemMessageType.SUCCESS);
		
		return true;
	}

}
