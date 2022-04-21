package de.jadr.chess;

import java.util.ArrayList;

public class PseudoArrayList<E> extends ArrayList<E>{
	private Object[] arr;	
	private int index = 0;
	private int size;
	
	private int openIndexes = 0;
	
	public PseudoArrayList(int size) {
		this.size = size;
	}
	
	
	@Override
	public boolean add(E e) {
		if(e == null)return false;
		if(index < size) {
			if(openIndexes > 0) {
				for (int i = 0;i < size;i++) {
					if(arr[i] == null) {
						arr[i] = e;
						openIndexes--;
						return true;
					}
				}
			}else {
				arr[index] = e;
				index++;
			}
			return true;
		}
		return false;
	}
	
	@Override
	public E set(int index, E element) {
		Object oval = arr[index];
		arr[index] = element;
		return (E) oval;
	}
	
	@Override
	public E remove(int index) {
		Object o = arr[index];
		arr[index] = null;
		
		if(o!=null) {
			openIndexes++;
			index--;
		}
		return (E) o;
	}
	
	@Override
	public boolean remove(Object o) {
		for(int i = 0;i < size;i++) {
			if(o.equals(arr[i])) {
				remove(i);				
				return true;
			}
		}
		return false;
	}
	
	@Override
	public int size() {
		return size;
	}
	
	@Override
	public void clear() {
		arr = new Object[size];
	}
	
	
}
