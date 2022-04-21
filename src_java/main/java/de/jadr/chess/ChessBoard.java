package de.jadr.chess;

import java.awt.Desktop;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import de.jadr.chess.ChessFigure.Type;

public class ChessBoard {

	public static final int WIDTH = 8, HEIGHT = 8;

	private ChessField[] fields;

	private LinkedList<ChessFigure> black = new LinkedList<ChessFigure>();
	private LinkedList<ChessFigure> white = new LinkedList<ChessFigure>();
	private ChessFigure blackKing;
	private ChessFigure whiteKing;

	private boolean isWhitesTurn = true;
	private int turns = 0;

	public ChessBoard() {
		setDefaultFigures();
	}
	
	public boolean isWhitesTurn() {
		return isWhitesTurn;
	}
	
	public void toggleTurn() {
		isWhitesTurn = !isWhitesTurn;
		System.out.println(generateFen());
	}

	private ChessBoard(boolean doo) {

	}
	
	public boolean isFiguryTypeUnderAttack(boolean isWhite, Type t) {
		ArrayList<Move> ms = getAvaibleMoves(!isWhite, false);
		for (Move move : ms) {
			if(move.getTarget() != null && move.getTarget().type() == t)return true;
		}
		
		return false;
	}

	public ChessBoard getCopy() {
		ChessBoard copy = new ChessBoard(true);
		copy.clear();
		for (int i = 0; i < fields.length; i++) {
			ChessFigure cf = fields[i].getFigure();
			if (cf != null) {
				copy.setFigure(i, cf.getCopy(copy));
			}
		}
		return copy;
	}

	public LinkedList<ChessFigure> whiteTracker() {
		return white;
	}

	public LinkedList<ChessFigure> blackTracker() {
		return black;
	}

	public ArrayList<Move> getAvaibleMoves(boolean isWhite, boolean doKingCheck) {
		ArrayList<Move> total = new ArrayList<Move>(100);
		LinkedList<ChessFigure> l;
		if (isWhite)
			l = white;
		else
			l = black;
		for (ChessFigure chessFigure : l) {
			total.addAll(chessFigure.getAvaibleMoves(doKingCheck));
		}
		return total;
	}

	public ArrayList<Move> getAttackMovesAtUs(boolean isWhite) {
		ArrayList<Move> total = new ArrayList<>(10);
		LinkedList<ChessFigure> l;
		if (!isWhite)
			l = white;
		else
			l = black;
		for (ChessFigure chessFigure : l) {
			ArrayList<Move> moves = chessFigure.getAvaibleMoves(true);
			for (Move move : moves) {
				if (move.getTarget() != null)
					total.add(move);
			}
		}
		return total;
	}
	/**
	 * @param kingPos
	 * @param kingSideRook
	 * @param queenSideRook
	 * @return boolean[] {canQueenSide, canRookSide}
	 */
	public boolean[] canCastle(int kingPos, int kingSideRook, int queenSideRook) {
		boolean castle[] = {false, false};
		ChessFigure k = getField(kingPos).getFigure();
		if (k != null && !k.alreadyMoved() && k.type() == Type.KING) {
			
			
			// Check queenSide
			ChessFigure queenRook = getField(queenSideRook).getFigure();
			if(queenRook != null && !queenRook.alreadyMoved() && queenRook.type() == Type.ROOK) {
				castle[0] = true;
			}
			// Check king side
			ChessFigure kingRook = getField(kingSideRook).getFigure();
			if (kingRook != null && !kingRook.alreadyMoved() && kingRook.type() == Type.ROOK) {
				castle[1] = true;
			}
		}
		return castle;
	}

	public boolean isMoveLegit(Move m) {
		if (m.getStart() < 0 || m.getStart() >= getFields().length)
			return false;
		ChessField f = getField(m.getStart());
		ChessFigure cf = f.getFigure();
		if (cf == null)
			return false;
		for (Move am : cf.getAvaibleMoves(true)) {
			if (am.equals(m))
				return true;
		}
		return false;
	}

	public void clear() {
		fields = new ChessField[WIDTH * HEIGHT];
		black.clear();
		white.clear();
		
		boolean white = true;
		for (int x = 0; x < WIDTH; x++) {
			for (int y = 0; y < HEIGHT; y++) {
				int j = x + y * WIDTH;
				fields[j] = new ChessField(this, white, x + y * WIDTH);
				white = !white;
			}
			white = !white;
		}
	}

	public void setDefaultFigures() {
		loadFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	}
	
	public ChessFigure getKingOf(boolean isWhite) {
		return isWhite?whiteKing:blackKing;
	}
	

	private void setFigure(int i, ChessFigure cf) {
		if (cf.isWhite()) {
			white.add(cf);
			if(cf.type() == Type.KING)whiteKing = cf;
		} else {
			black.add(cf);
			if(cf.type() == Type.KING)blackKing = cf;
		}
		getField(i).setFigure(cf);
	}

	private void setFigure(int x, int row, ChessFigure cf) {
		setFigure(x + row * 8, cf);
	}

	public void setRow(int row, ChessFigure.Type type, boolean white) {
		for (int x = 0; x < WIDTH; x++) {
			setFigure(x, row, new ChessFigure(type, x + row * WIDTH, white, this));
		}
	}

	public void setRow(int row, boolean white, ChessFigure.Type... types) {
		for (int x = 0; x < types.length; x++) {
			setFigure(x, row, new ChessFigure(types[x], x + row * WIDTH, white, this));
		}
	}

	public ChessField getField(int i) {
		return fields[i];
	}

	public ChessField getField(int x, int y) {
		if (y >= 8 || y < 0 || x >= 8 || x < 0)
			return null;
		return getField(x + y * WIDTH);
	}

	public ChessField[] getFields() {
		return fields;
	}
	
	private boolean isNumber(char c) {
		for (char d : "0123456789".toCharArray()) if(d==c)return true;
		return false;
	}
	
	public void loadFen(String fen) {
		clear();
		String[] fenAreas = fen.split(" ");
		
		
		String[] figRows = fenAreas[0].split("/");
		for(int y = 0;y < figRows.length;y++) {
			char[] row = figRows[y].toCharArray();
			int x = 0;
			for(int i = 0;i < row.length;i++) {
				char c = row[i];
				if(isNumber(c)) {
					int j = Character.getNumericValue(c);
					x+=j-1;
				}else {
					Type type = ChessFigure.Type.fromFenChar(c);
					setFigure(x,y,new ChessFigure(type, x+y*WIDTH, Character.isUpperCase(c), this));
				}
				x++;
			}
		}
		// [rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR, w, KQkq, -, 0, 1]
		isWhitesTurn = fenAreas[1].equals("w");
		
		String castle = fenAreas[2];
		System.out.println("Castle: [" + castle + "]");
		if(!castle.equals("-")) {
			
			if(!castle.contains("K")) {
				ChessFigure cf = getField(63).getFigure();
				if(cf != null && cf.type() == Type.ROOK)cf.setAlreadyMoved();
			}
			if(!castle.contains("Q")) {
				ChessFigure cf = getField(56).getFigure();
				if(cf != null && cf.type() == Type.ROOK)cf.setAlreadyMoved();
			}
			
			if(!castle.contains("k")) {
				ChessFigure cf = getField(7).getFigure();
				if(cf != null && cf.type() == Type.ROOK)cf.setAlreadyMoved();
			}
			
			if(!castle.contains("q")) {
				ChessFigure cf = getField(0).getFigure();
				if(cf != null && cf.type() == Type.ROOK)cf.setAlreadyMoved();
			}
			
		}
		//turns = Integer.valueOf(fenAreas[5]);
	}

	public String generateFen() {
		StringBuilder fen = new StringBuilder(32);
		for (int y = 0; y < HEIGHT; y++) {
			StringBuilder line = new StringBuilder(4);
			int startOffset = 0;
			for (int x = 0; x < WIDTH; x++) {
				ChessFigure cf = getField(x, y).getFigure();
				if (cf == null) {
					startOffset++;
				} else {
					if (startOffset > 0) {
						line.append(startOffset);
						startOffset = 0;
					}
					line.append(cf.getFenChar());
				}
			}
			if (startOffset > 0)
				line.append(startOffset);
			line.append('/');
			fen.append(line);
		}

		// / from last line
		fen.deleteCharAt(fen.length() - 1);
		// Whos turn
		fen.append(" " + (isWhitesTurn ? 'w' : 'b') + " ");
		
		//white king
		boolean[] whiteCastle = canCastle(60, 63, 56);
		boolean[] blackCastle = canCastle(4,7,0);
		
		if(whiteCastle[1])fen.append('K');
		if(whiteCastle[0])fen.append('Q');
		if(blackCastle[1])fen.append('k');
		if(blackCastle[0])fen.append('q');
		
		fen.append(" - 0 " + turns);
		return fen.toString();
	}
	
//	public void openToLichess() {
//		try {
//			Desktop.getDesktop().browse(new URI("https://lichess.org/editor/"+generateFen().replaceAll(" ", "_")));
//		} catch (Exception e) {
//			e.printStackTrace();
//		}
//	}

}
