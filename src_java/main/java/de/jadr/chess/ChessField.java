package de.jadr.chess;

public class ChessField {

	private ChessBoard board;
	private ChessFigure figure;
	private boolean isWhite;
	private int pos;
	
	public ChessField(ChessBoard chessBoard, boolean isWhite, int pos) {
		this.board = chessBoard;
		this.isWhite = isWhite;
		this.pos = pos;
	}
	
	public int getPos() {
		return pos;
	}
	
	public boolean isFieldWhite() {
		return this.isWhite;
	}
	
	public ChessBoard getBoard() {
		return this.board;
	}
	
	public void setFigure(ChessFigure figure) {
		this.figure = figure;
	}
	
	public ChessFigure getFigure() {
		return this.figure;
	}
}
