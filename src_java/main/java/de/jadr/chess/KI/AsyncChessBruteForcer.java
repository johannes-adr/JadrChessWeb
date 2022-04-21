package de.jadr.chess.KI;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import de.jadr.chess.ChessBoard;
import de.jadr.chess.ChessFigure;
import de.jadr.chess.ChessFigure.Type;
import de.jadr.chess.Move;

public class AsyncChessBruteForcer {

	private boolean isWhite;

	private ExecutorService EXECUTOR = Executors.newCachedThreadPool();

	private ChessBoard cb;
	private int depth;

	public AsyncChessBruteForcer(ChessBoard cb, boolean isWhite, int depth) {
		this.cb = cb;
		this.isWhite = isWhite;
		if (isWhite) {
			cb.whiteTracker();
		} else {
			cb.blackTracker();
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
		ArrayList<Move> moves = cb.getAvaibleMoves(isWhite, true);
		ArrayList<Future<Object[]>> results = new ArrayList<>();
		for (Move move : moves) {
			Future<Object[]> result = EXECUTOR.submit(new Callable<Object[]>() {
				@Override
				public Object[] call() throws Exception {
					ChessBoard clonedBoard = cb.getCopy();
					Runnable undo = move.cloneToBoard(clonedBoard).exec();
					int currentBoard = doMiniMax(!isWhite, depth, clonedBoard);
					undo.run();
					return new Object[] { currentBoard, move };
				}
			});
			results.add(result);
		}

		for (Future<Object[]> future : results) {
			try {
				Object[] result = future.get();
				int currentBoard = (int) result[0];
				Move move = (Move) result[1];
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
			} catch (Exception e) {
				e.printStackTrace();
			}
		}
		if(bestMove != null) {
			bestMove.exec();
		}
		
		//System.out.println("Doing move: " + bestMove + " with worth: " + extrema + " (" + calculations + ")");
		return new KiMoveResult(bestMove);
	}

	

	private int worthOfSite(boolean isWhite, ChessBoard cb) {
		int totalWorth = 0;
		LinkedList<ChessFigure> map;
		if (isWhite) {
			map = cb.whiteTracker();
		} else {
			map = cb.blackTracker();
		}
		for (ChessFigure chessFigure : map) {
			Type type = chessFigure.type();
			totalWorth += type.worth()+type.evaluatePosition(isWhite,chessFigure);
			
		}
		return totalWorth;
	}

	private int max(int a, int b) {
		return a > b ? a : b;
	}

	private int min(int a, int b) {
		return a < b ? a : b;
	}

	private int doMiniMax(boolean isWhite, int depth, ChessBoard cb) {
		if (isWhite) {
			return miniMax(isWhite, depth, Integer.MIN_VALUE, Integer.MAX_VALUE, cb);
		} else {
			return miniMax(isWhite, depth, Integer.MAX_VALUE, Integer.MIN_VALUE, cb);
		}
	}

	private ArrayList<Move> sortFromBest(ArrayList<Move> moves) {
		ArrayList<Move> primaryList = new ArrayList<Move>(moves.size() / 2);
		ArrayList<Move> secondaryList = new ArrayList<Move>(moves.size() / 2);
		for (Move move : moves) {
			if (move.getTarget() != null || move.getFlagData() != null) {
				primaryList.add(move);
			} else {
				secondaryList.add(move);
			}
		}
		primaryList.addAll(secondaryList);
		return moves;
	}

	// White want to max, black wants to min
	private int miniMax(boolean isWhite, int depth, int alpha, int beta, ChessBoard cb) {
		if (depth == 0) {
			calculations++;
			return evalSite(cb);
		}
		int extrema = isWhite ? Integer.MIN_VALUE : Integer.MAX_VALUE;
		ArrayList<Move> moves = sortFromBest(cb.getAvaibleMoves(isWhite, true));
		if (moves.size() == 0) {
			if (cb.isFiguryTypeUnderAttack(isWhite, Type.KING)) {
				
				return (extrema/10) * depth + 1;
			}
			return 0;
		}
		
		for (Move move : moves) {
			Runnable undo = move.exec();
			
			int currentBoard = miniMax(!isWhite, depth - 1, alpha, beta, cb);
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

	/**
	 * @param cb
	 * @return the difference from the sum of black pieces to the white ones
	 */
	public int evalSite(ChessBoard cb) {
		int ourWorth = worthOfSite(true, cb);
		int enemyWorth = worthOfSite(false, cb);
		int diff = ourWorth - enemyWorth;
		return diff;
	}

	public boolean isWhite() {
		return isWhite;
	}

}