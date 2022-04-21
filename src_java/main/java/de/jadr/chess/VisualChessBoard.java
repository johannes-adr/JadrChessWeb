package de.jadr.chess;

import java.awt.Color;
import java.awt.Graphics2D;
import java.awt.Point;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.awt.image.BufferedImage;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Locale;
import java.util.concurrent.atomic.AtomicInteger;

import de.j2d.core.Game;
import de.jadr.chess.KI.AsyncChessBruteForcer;
import de.jadr.chess.KI.ChessBruteForcer;

public class VisualChessBoard extends Game implements Runnable {

	private ChessBoard board;

	private AsyncChessBruteForcer kiBlack;
	private AsyncChessBruteForcer kiWhite;

	private static final int SIZE = 100, CELLW = SIZE, CELLH = SIZE;

	public VisualChessBoard(String title, ChessBoard cb) {
		super(title, CELLW * ChessBoard.WIDTH + 15, CELLH * ChessBoard.HEIGHT + 45);
		this.board = cb;
		kiWhite = new AsyncChessBruteForcer(board, true, 4);
		kiBlack = new AsyncChessBruteForcer(board, false, 4);
	}

	public static void main(String[] args) {
		ChessBoard cb = new ChessBoard();
		cb.loadFen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w - - 0 1");
		new VisualChessBoard("ChessKI", cb).start();
	}

	LinkedList<Runnable> undos = new LinkedList<>();

	public String formatStr(float val) {
		return String.format(Locale.CANADA, "%,.2f", val);
	}

	@Override
	protected void onStart() {
		getWindow().getCanvas().addKeyListener(new KeyListener() {

			@Override
			public void keyTyped(KeyEvent e) {
//				Runnable um = undos.pollLast();
//				if (um != null)
//					um.run();
			}

			@Override
			public void keyReleased(KeyEvent e) {
				// TODO Auto-generated method stub

			}

			@Override
			public void keyPressed(KeyEvent e) {
				// TODO Auto-generated method stub

			}
		});

//		new Thread(() -> {
//			try {
//				while (true) {
//					Thread.sleep(500L);
//					kiWhite.doTurn();
//					Thread.sleep(500L);
//					kiBlack.doTurn();
//				}
//			} catch (Exception e) {
//				e.printStackTrace();
//			}
//		}).start();

	}

	private boolean dragged = false;
	private ChessFigure dragFigure = null;

	@Override
	protected void onUpdate() {
	}

	private static final Color BLACK_FIELD = Color.decode("#b88c64"), WHITE_FIELD = Color.decode("#f8dcb4"),
			AVAIBLE_MOVE = new Color(46, 204, 113, 100);

	@Override
	protected void render(Graphics2D g) {
		for (int x = 0; x < ChessBoard.WIDTH; x++) {
			for (int y = 0; y < ChessBoard.HEIGHT; y++) {
				ChessField cf = board.getField(x, y);
				if (cf.isFieldWhite()) {
					g.setColor(WHITE_FIELD);
				} else {
					g.setColor(BLACK_FIELD);
				}
				g.fillRect(x * CELLW, y * CELLH, CELLW, CELLH);
				g.setColor(Color.black);
				g.drawString("" + (x + y * 8), x * CELLW + 5, y * CELLH + (int) (CELLH * 0.95));
				ChessFigure f = cf.getFigure();
				if (f != null && !f.equals(dragFigure)) {
					g.drawImage(ChessFigureImage.getImage(f.isWhite(), f.type()), x * CELLW, y * CELLH, CELLW, CELLH,
							null);
				}
			}
		}

		Point p = super.getWindow().mouseLocPoint;
		ChessField cf = board.getField(p.x / CELLW, p.y / CELLH);
		if (isMouseKeyPressed(Window.MOUSE_LEFT)) {
			if (!dragged) {
				this.dragFigure = cf.getFigure();
				if (dragFigure != null && !dragFigure.isWhite())
					dragFigure = null;
			}
			dragged = true;
		} else {
			if (dragged) {
				if (cf != null && dragFigure != null) {
					ArrayList<Move> moves = dragFigure.getAvaibleMoves(true);
					for (Move move : moves) {
						// Just move if possible
						if (move.getEnd() == cf.getPos()) {
							undos.add(dragFigure.doMove(move));
							kiBlack.doTurn();
							break;
						}
					}

				}
				this.dragFigure = null;
			}
			dragged = false;
		}
		if (dragged && dragFigure != null) {
			BufferedImage bi = ChessFigureImage.getImage(dragFigure.isWhite(), dragFigure.type());
			ArrayList<Move> moves = dragFigure.getAvaibleMoves(true);
			drawMoves(moves, g);
			g.drawImage(bi, p.x - CELLW / 2, p.y - CELLW / 2, CELLW, CELLH, null);
		}
	}

	private void drawMoves(ArrayList<Move> moves, Graphics2D g) {
		g.setColor(AVAIBLE_MOVE);
		for (Move move : moves) {
			int x = move.getEnd() % 8;
			int y = move.getEnd() / 8;
			g.fillRect(x * CELLW, y * CELLH, CELLW, CELLH);
		}
	}
}
