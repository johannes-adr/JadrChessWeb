package de.jadr;

import java.util.ArrayList;

import org.java_websocket.WebSocket;

public class CustomWebSocketClient {
	

	
	public static interface MessageRunnable{
		public void run(String in);
	}
	
	private final WebSocket ws;
	private ArrayList<Runnable> onClose = new ArrayList<Runnable>();
	private ArrayList<Runnable> onOpen = new ArrayList<Runnable>();
	
	public CustomWebSocketClient(WebSocket ws) {
		this.ws = ws;
	}
	
	public void addOnClose(Runnable r) {
		this.onClose.add(r);
	}

	
	public WebSocket getWebSocket() {
		return ws;
	}
	
	public void onOpen() {
		onOpen.forEach(r->r.run());
	}
	
	public void onClose() {
		onClose.forEach(r->r.run());
	}
	
}
