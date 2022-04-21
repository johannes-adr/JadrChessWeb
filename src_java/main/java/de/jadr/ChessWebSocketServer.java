package de.jadr;

import java.net.InetSocketAddress;
import java.util.HashMap;
import java.util.UUID;

import org.java_websocket.WebSocket;
import org.java_websocket.handshake.ClientHandshake;
import org.java_websocket.server.WebSocketServer;
import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

public class ChessWebSocketServer extends WebSocketServer {

	public HashMap<UUID, ChessLobby> lobbys = new HashMap<>();
	public HashMap<WebSocket, CustomWebSocketClient> cws = new HashMap<WebSocket, CustomWebSocketClient>();

	public ChessWebSocketServer(int port) {
		super(new InetSocketAddress(port));
	}

	@Override
	public void onOpen(WebSocket conn, ClientHandshake handshake) {
		CustomWebSocketClient cwsc = new CustomWebSocketClient(conn);
		cws.put(conn, cwsc);
		cwsc.onOpen();
	}

	@Override
	public void onClose(WebSocket conn, int code, String reason, boolean remote) {
		CustomWebSocketClient cwsc = cws.get(conn);
		cws.remove(conn);
		cwsc.onClose();
	}

	@Override
	public void onMessage(WebSocket conn, String message) {
		try {
			JSONObject ret = new JSONObject();
			JSONObject jo = new JSONObject(message);
			String type = jo.getString("type");
			switch (type) {
			case "createLobby": {
				if (lobbys.size() > 1000) {
					ret.put("error", "too many lobbys");
					conn.send(ret.toString());
				} else {
					System.out.println("creating lobby");
					UUID uuid = UUID.randomUUID();

					ChessLobby cl = new ChessLobby(uuid, cws.get(conn), this);
					lobbys.put(uuid, cl);

					ret.put("type", "lobbycreated");
					ret.put("lobbyid", uuid.toString());
					System.out.println(ret.toString());
					conn.send(ret.toString());
				}
				return;
			}
			case "joinLobby": {
				UUID uuid = UUID.fromString(jo.getString("lobbyid"));
				lobbys.get(uuid).join(cws.get(conn));
				return;
			}
			case "doMove": {
				JSONArray arr = jo.getJSONArray("args");
				if (!jo.has("lobbyid") || !(jo.get("lobbyid") instanceof String))
					return;
				UUID uuid = UUID.fromString(jo.getString("lobbyid"));
				lobbys.get(uuid).doMove(arr.getJSONObject(1), cws.get(conn));
				return;
			}
			case "chatmessage": {
				UUID uuid = UUID.fromString(jo.getString("lobbyid"));
				lobbys.get(uuid).sendMessage(jo, cws.get(conn));
				return;
			}
			default:
				ret.put("error", "unknown packet");
				conn.send(ret.toString());
				return;
			}
		} catch (JSONException e) {
			e.printStackTrace();
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

	@Override
	public void onError(WebSocket conn, Exception ex) {
		// TODO Auto-generated method stub

	}

	@Override
	public void onStart() {
	}

}
