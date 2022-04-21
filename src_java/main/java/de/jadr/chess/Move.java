package de.jadr.chess;

public class Move {
	
	@Override
	public String toString() {
		return "Move [start=" + start + ", end=" + end + " target=" + target + ", flagData="
				+ flagData + "]";
	}

	public static enum Flag{
		CASTLE,PROMOTE
	}
	
	public static class FlagData{
		private final Flag flag;
		private final Object data;
		
		public Flag getFlag() {
			return flag;
		}
		
		public Object getData() {
			return data;
		}
		public FlagData(Flag flag, Object data) {
			this.flag = flag;
			this.data = data;
		}
	}
	
	private int flatWorth = 0;
	
	private int start;
	private int end;
	private ChessFigure target;
	private ChessFigure executor;
	
	private FlagData flagData;
	
	public Move(int start, int end, ChessFigure executor) {
		this.start = start;
		this.end = end;
		this.executor = executor;
	}
	
	public void setFlatWorth(int i) {
		flatWorth=i;
	}
	
	public int getFlatWorth() {
		return flatWorth;
	}
	
	public ChessFigure getExecutor() {
		return this.executor;
	}
	
	public Move setTarget(ChessFigure target) {
		this.target = target;
		return this;
	}
	
	public Move setFlagData(FlagData flagData) {
		this.flagData = flagData;
		return this;
	}
	
	public Runnable exec() {
		return executor.doMove(this);
	}
	
	public ChessFigure getTarget() {
		return target;
	}
	
	public int getStart() {
		return this.start;
	}
	
	public int getEnd() {
		return this.end;
	}
	
	public FlagData getFlagData() {
		return this.flagData;
	}
	
	@Override
	public boolean equals(Object obj) {
		if(!(obj instanceof Move))return false;
		Move m = (Move) obj;
		if(m.start == start && m.end == end && m.getExecutor().equals(executor))return true;
		return false;
	}

	public Move cloneToBoard(ChessBoard cb) {
		Move m = new Move(start, end, cb.getField(start).getFigure());
		ChessFigure target = cb.getField(end).getFigure();
		if(target != null) {
			m.target = target;
		}
		m.flagData = flagData;
		
		return m;
	}
	
}
