package de.jadr.chess;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Objects;

import org.json.JSONArray;

import de.jadr.chess.ChessFigure.Type;
import de.jadr.chess.Move.Flag;
import de.jadr.chess.Move.FlagData;

public class ChessFigure {

	public static enum Type {
		PAWN(100,false, 'p', 0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5, 10,
				25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20, -20, 10, 10,
				5, 0, 0, 0, 0, 0, 0, 0, 0),
		KNIGHT(300,false, 'n', -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
				0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10, 5, -30,
				-40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50),
		BISHOP(300,false, 'b', -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
				-10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10, -10,
				-10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20),
		ROOK(500,false, 'r', 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
				0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 5, 5,
				0, 0, 0),
		QUEEN(900,false, 'q', -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
				-5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0, 0, 0,
				-10, -20, -10, -10, -5, -5, -10, -10, -20),
		KING(90_000,true, 'k', -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
				-50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30, -30, -20,
				-10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20);

		private final int worth;
		private final int[] whiteValueMap;
		private final int[] blackValueMap;
		private final char blackFenChar;
		private final boolean isKing;
		Type(int i,boolean isKing, char fenChar, int... valueMap) {
			this.worth = i;
			this.whiteValueMap = valueMap;
			this.blackFenChar = fenChar;
			this.isKing = isKing;
			int[] m = new int[valueMap.length];

			for (int x = 0; x < 8; x++) {
				for (int y = 0; y < 8; y++) {
					int ly = 7 - y;

					m[x + y * 8] = valueMap[x + ly * 8];
				}
			}

			this.blackValueMap = m;
		}

		public static Type fromFenChar(char c) {
			c = Character.toLowerCase(c);
			for (Type t : Type.values()) {
				if (t.blackFenChar == c)
					return t;
			}
			return null;
		}

		public char blackFenChar() {
			return blackFenChar;
		}

		public char whiteFenChar() {
			return Character.toUpperCase(blackFenChar);
		}

		public int worth() {
			return this.worth;
		}

		public int[] blackValueMap() {
			return blackValueMap;
		}

		public int[] whiteValueMap() {
			return whiteValueMap;
		}
		
		public int evaluatePosition(boolean isWhite, ChessFigure cf) {
			int worth = 0;
			if (isWhite) {
				worth += whiteValueMap()[cf.getPos()];
			} else {
				worth += blackValueMap()[cf.getPos()];
			}
			if(isKing) {
				ChessFigure ek = cf.board.getKingOf(!isWhite);
				int dist = (int) (Math.sqrt(Math.pow((cf.x-ek.x),2)+Math.pow((cf.y-ek.y),2)));
				worth+=(10-dist)*10;
				
			}
			return worth;
		}
	}

	private ChessFigure thiz = this;
	private Type type;
	private final boolean isWhite;
	private int pos;

	private boolean alreadyMoved = false;

	private int x;
	private int y;

	private ChessBoard board;

	public ChessFigure(Type type, int pos, boolean isWhite, ChessBoard board) {
		this.type = type;
		this.isWhite = isWhite;
		this.board = board;
		setPos(pos);
	}

	public void setAlreadyMoved() {
		this.alreadyMoved = true;
	}

	public boolean alreadyMoved() {
		return this.alreadyMoved;
	}

	@Override
	public String toString() {
		return "{" + type + "| x: " + x + " y: " + y + "}";
	}

	public Runnable kill() {
		getField(x, y).setFigure(null);
		if (isWhite) {
			board.whiteTracker().remove(this);
		} else {
			board.blackTracker().remove(this);
		}
		return new Runnable() {
			@Override
			public void run() {
				getField(x, y).setFigure(thiz);
				if (isWhite) {
					board.whiteTracker().add(thiz);
				} else {
					board.blackTracker().add(thiz);
				}
			}
		};
	}

	public Runnable doMove(Move m) {
		if (pos != m.getStart())
			throw new RuntimeException(
					"Unable to move " + this + " - requested move start location is not the same " + m);
		boolean movedBefore = this.alreadyMoved;
		this.alreadyMoved = true;

		board.getField(pos).setFigure(null);
		ChessField targetField = board.getField(m.getEnd());
		ChessFigure target = targetField.getFigure();

		Runnable uknotfinal = null;
		if (target != null)
			uknotfinal = target.kill();
		Runnable undoKill = uknotfinal;
		targetField.setFigure(this);
		if (m.getFlagData() != null) {
			FlagData fd = m.getFlagData();
			switch (fd.getFlag()) {
			case PROMOTE:
				this.type = Type.QUEEN;
				break;
			case CASTLE:
				Object dataRaw = fd.getData();
				// Data is a jsonarray if it is a packet from network
				if (dataRaw instanceof JSONArray) {
					JSONArray jarr = (JSONArray) dataRaw;
					int[] da = new int[jarr.length()];
					for (int i = 0; i < jarr.length(); i++) {
						da[i] = jarr.getInt(i) % 8;
					}
					dataRaw = da;
				}
				int[] data = (int[]) dataRaw;
				ChessFigure cf = getField(data[0], y).getFigure();
				cf.doMove(new Move(data[0] + y * 8, data[1] + y * 8, cf));
				break;
			}

		}
		setPos(m.getEnd());
		return new Runnable() {
			@Override
			public void run() {
				if (undoKill != null)
					undoKill.run();
				else
					board.getField(m.getEnd()).setFigure(null);
				alreadyMoved = movedBefore;
				board.getField(m.getStart()).setFigure(thiz);
				if (m.getFlagData() != null) {
					FlagData fd = m.getFlagData();
					switch (fd.getFlag()) {
					case PROMOTE:
						thiz.type = Type.PAWN;
						break;
					case CASTLE:
						int[] data = (int[]) fd.getData();
						ChessFigure cf = getField(data[1], y).getFigure();
						cf.doMove(new Move(data[1] + y * 8, data[0] + y * 8, cf));
						cf.alreadyMoved = false;
						break;
					}
				}
				setPos(m.getStart());
			}
		};

	}

	public Type type() {
		return this.type;
	}

	public void setType(Type t) {
		this.type = t;
	}

	public void setPos(int i) {
		this.pos = i;
		this.y = i / 8;
		this.x = i % 8;
	}

	public int getPos() {
		return this.pos;
	}

	public int getY() {
		return y;
	}

	public int getX() {
		return x;
	}

	public boolean isWhite() {
		return this.isWhite;
	}

	public ArrayList<Move> getAvaibleMoves(boolean doKingCheck) {
		ArrayList<Move> moves = new ArrayList<Move>(6);
		switch (type) {
		case PAWN:
			addPawn(moves, doKingCheck);
			break;
		case ROOK:
			addRook(moves, doKingCheck);
			break;
		case BISHOP:
			addBishop(moves, doKingCheck);
			break;
		case KNIGHT:
			addKnight(moves, doKingCheck);
			break;
		case QUEEN:
			addBishop(moves, doKingCheck);
			addRook(moves, doKingCheck);
			break;
		case KING:
			addKing(moves, doKingCheck);
		default:
			break;
		}

		return moves;
	}

	private void addMove(ArrayList<Move> moves, int endPos, FlagData flagData, boolean doKingCheck) {
		ChessFigure f = board.getField(endPos).getFigure();

		Move move = new Move(pos, endPos, this);
		if (f != null) {
			if (f.isWhite == isWhite)
				return;
			move.setTarget(f);
		}
		if (flagData != null)
			move.setFlagData(flagData);
		if(doKingCheck) {
			Runnable r = move.exec();
			if(board.isFiguryTypeUnderAttack(isWhite, Type.KING)) {
				r.run();
				return;
			}
			r.run();
		}
		
		moves.add(move);
	}

	private void addMove(ArrayList<Move> moves, int x, int y, FlagData flagData, boolean doKingCheck) {
		if (!xInBounds(x) || y < 0 || y > 7)
			return;
		addMove(moves, x + y * 8, flagData,doKingCheck);
	}

	private void addMove(ArrayList<Move> moves, int x, int y,boolean doKingCheck) {
		addMove(moves, x, y, null,doKingCheck);
	}

	private ChessField getField(int x, int y) {
		return this.board.getField(x, y);
	}

	private void addPawn(ArrayList<Move> moves, boolean doKingCheck) {
		// One field which represents the direction the pawn can move
		int of = isWhite ? -1 : 1;

		// One field in front
		int yof = y + of;
		FlagData promotion = null;
		if (yof == 0 || yof == 7) {
			promotion = new FlagData(Flag.PROMOTE, null);
		}

		// Sinle jump
		if (fieldEmpty(x, yof)) {
			addMove(moves, x, yof, promotion,doKingCheck);
			// Double jump
			int dof = y + of * 2;
			if (!alreadyMoved && fieldEmpty(x, dof))
				addMove(moves, x, dof,doKingCheck);
		}

		// Right / Left attack
		if (!fieldEmpty(x + 1, yof))
			addMove(moves, x + 1, yof, promotion,doKingCheck);
		if (!fieldEmpty(x - 1, yof))
			addMove(moves, x - 1, yof, promotion,doKingCheck);

	}

	private static final int[] KNIGHT_DELTAS = { -2, -1, -2, +1, +2, -1, +2, +1, -1, -2, -1, +2, +1, -2, +1, +2 };

	private void addKnight(ArrayList<Move> moves, boolean doKingCheck) {
		for (int i = 0; i < KNIGHT_DELTAS.length; i += 2) {
			int x = KNIGHT_DELTAS[i];
			int y = KNIGHT_DELTAS[i + 1];
			addMove(moves, this.x + x, this.y + y,doKingCheck);
		}
	}

	private boolean xInBounds(int x) {
		return !(x > 7 || x < 0);
	}

	private void addBishop(ArrayList<Move> moves, boolean doKingCheck) {
		{
			// RIGHT DOWN
			int i = 1;
			for (int y = this.y + 1; y < 8; y++) {
				int x = this.x + i++;
				if (!xInBounds(x))
					break;
				addMove(moves, x, y,doKingCheck);
				if (!fieldEmpty(x, y))
					break;
			}
		}

		{
			// LEFT DOWN
			int i = -1;
			for (int y = this.y + 1; y < 8; y++) {
				int x = this.x + i--;
				if (!xInBounds(x))
					break;
				addMove(moves, x, y,doKingCheck);
				if (!fieldEmpty(x, y))
					break;
			}
		}

		{
			// RIGHT UP
			int i = 1;
			for (int y = this.y - 1; y >= 0; y--) {
				int x = this.x + i++;
				if (!xInBounds(x))
					break;
				addMove(moves, x, y,doKingCheck);
				if (!fieldEmpty(x, y))
					break;
			}
		}

		{
			// LEFT UP
			int i = -1;
			for (int y = this.y - 1; y >= 0; y--) {
				int x = this.x + i--;
				if (!xInBounds(x))
					break;
				addMove(moves, x, y,doKingCheck);
				if (!fieldEmpty(x, y))
					break;
			}
		}

	}

	private void addRook(ArrayList<Move> moves, boolean doKingCheck) {
		// DOWN
		for (int y = this.y + 1; y < 8; y++) {
			addMove(moves, x, y,doKingCheck);
			if (!fieldEmpty(x, y))
				break;
		}

		// UP
		for (int y = this.y - 1; y >= 0; y--) {
			addMove(moves, x, y,doKingCheck);
			if (!fieldEmpty(x, y))
				break;
		}

		// RIGHT
		for (int x = this.x + 1; x < 8; x++) {
			addMove(moves, x, y,doKingCheck);
			if (!fieldEmpty(x, y))
				break;
		}

		// LEFT
		for (int x = this.x - 1; x >= 0; x--) {
			addMove(moves, x, y,doKingCheck);
			if (!fieldEmpty(x, y))
				break;
		}
	}

	private boolean fieldEmpty(int x, int y) {
		ChessField f = getField(x, y);
		if (f == null)
			return false;
		return f.getFigure() == null;
	}

	private void addKing(ArrayList<Move> moves, boolean doKingCheck) {
		for (int x = this.x - 1; x <= this.x + 1; x++) {
			for (int y = this.y - 1; y <= this.y + 1; y++) {
				addMove(moves, x, y,doKingCheck);
			}
		}

		// Castle
		if (!alreadyMoved) {
			if (x != 4)
				return;
			ChessFigure rookRight = getField(x + 3, y).getFigure();
			ChessFigure rookLeft = getField(x - 4, y).getFigure();
			if (rookRight != null && !rookRight.alreadyMoved && rookRight.type == Type.ROOK) {
				if (fieldEmpty(x + 1, y) && fieldEmpty(x + 2, y)) {
					addMove(moves, x + 2, y, new FlagData(Flag.CASTLE, new int[] { x + 3, x + 1 }),doKingCheck);
				}
			}

			if (rookLeft != null && !rookLeft.alreadyMoved && rookLeft.type == Type.ROOK) {
				if (fieldEmpty(x - 1, y) && fieldEmpty(x - 2, y) && fieldEmpty(x - 3, y)) {
					addMove(moves, x - 2, y, new FlagData(Flag.CASTLE, new int[] { x - 4, x - 1 }),doKingCheck);
				}
			}
		}
	}

	public char getFenChar() {
		if (isWhite) {
			return type.whiteFenChar();
		} else {
			return type.blackFenChar();
		}
	}

	public ChessFigure getCopy(ChessBoard cb) {
		ChessFigure copy = new ChessFigure(type, pos, isWhite, cb);
		copy.alreadyMoved = alreadyMoved;
		return copy;
	}

}
