package de.jadr.chess.KI;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.concurrent.Callable;
import java.util.concurrent.ForkJoinPool;

import javax.naming.ldap.ExtendedRequest;

import de.jadr.chess.ChessBoard;
import de.jadr.chess.ChessFigure;
import de.jadr.chess.ChessFigure.Type;
import de.jadr.chess.Move;

public class ChessBruteForcer {

	private boolean isWhite;

	private LinkedList<ChessFigure> ownFigures;
	private LinkedList<ChessFigure> enemyFigures;

	protected int depth;
	protected ChessBoard cb;

	public ChessBruteForcer(ChessBoard cb, boolean isWhite, int depth) {
		this.cb = cb;
		this.isWhite = isWhite;
		if (isWhite) {
			ownFigures = cb.whiteTracker();
		} else {
			ownFigures = cb.blackTracker();
		}
		this.depth = depth;
	}

	private int calculations = 0;

	public class KiMoveResult {
		private final int calcs = calculations;
		private final Move move;

		public KiMoveResult(Move m) {
			this.move = m;
		}

		public int getCalculations() {
			return calcs;
		}

		public Move getMove() {
			return move;
		}
	}

	public KiMoveResult doTurn() {
		calculations = 0;
		int extrema = isWhite ? Integer.MIN_VALUE : Integer.MAX_VALUE;

		Move bestMove = null;
		ArrayList<Move> moves = cb.getAvaibleMoves(isWhite,true);

		for (Move move : moves) {
			Runnable undo = move.exec();
			int currentBoard = doMiniMax(!isWhite, depth);
			if (isWhite) {
				if (currentBoard > extrema) {
					extrema = currentBoard;
					bestMove = move;
				}
			} else {
				if (currentBoard < extrema) {
					extrema = currentBoard;
					bestMove = move;
				}
			}
			undo.run();
		}
		bestMove.exec();
		System.out.println("Doing move: " + bestMove + " with worth: " + extrema + " (" + calculations + ")");
		return new KiMoveResult(bestMove);
	}

	private int worthOfSite(boolean isWhite) {
		int totalWorth = 0;
		LinkedList<ChessFigure> map;
		if (isWhite) {
			map = cb.whiteTracker();
		} else {
			map = cb.blackTracker();
		}
		for (ChessFigure chessFigure : map) {
			Type type = chessFigure.type();
			totalWorth += type.worth();
			if(isWhite) {
				totalWorth+=type.whiteValueMap()[chessFigure.getPos()];
			}else{
				totalWorth+=type.blackValueMap()[chessFigure.getPos()];
			}
		}
		return totalWorth;
	}

	private int max(int a, int b) {
		return a > b ? a : b;
	}

	private int min(int a, int b) {
		return a < b ? a : b;
	}

	private int doMiniMax(boolean isWhite, int depth) {
		if (isWhite) {
			return miniMax(isWhite, depth, Integer.MIN_VALUE, Integer.MAX_VALUE);
		} else {
			return miniMax(isWhite, depth, Integer.MAX_VALUE, Integer.MIN_VALUE);
		}

	}

	private ArrayList<Move> sortFromBest(ArrayList<Move> moves) {
		ArrayList<Move> primaryList = new ArrayList<Move>(moves.size()/2);
		ArrayList<Move> secondaryList = new ArrayList<Move>(moves.size()/2);
		for (Move move : moves) {
			if(move.getTarget() != null) {
				primaryList.add(move);
			}else {
				secondaryList.add(move);
			}
		}
		primaryList.addAll(secondaryList);
		return moves;
	}

	// White want to max
	private int miniMax(boolean isWhite, int depth, int alpha, int beta) {
		if (depth == 0) {
			calculations++;
			return evalSite();
		}
		ArrayList<Move> moves = sortFromBest(cb.getAvaibleMoves(isWhite,false));
		int extrema = isWhite ? Integer.MIN_VALUE : Integer.MAX_VALUE;
		for (Move move : moves) {
			Runnable undo = move.exec();

			int currentBoard = miniMax(!isWhite, depth - 1, alpha, beta);
			if (isWhite) {
				extrema = max(extrema, currentBoard);
				alpha = max(alpha, currentBoard);
				if (beta <= alpha) {
					undo.run();
					break;
				}

			} else {
				extrema = min(extrema, currentBoard);
				beta = min(beta, currentBoard);
				if (beta <= alpha) {
					undo.run();
					break;
				}
			}

			undo.run();
		}
		return extrema;
	}

	public int evalSite() {
		int ourWorth = worthOfSite(true);
		int enemyWorth = worthOfSite(false);
		int diff = ourWorth - enemyWorth;
		return diff;
	}

	// White wants min,

}