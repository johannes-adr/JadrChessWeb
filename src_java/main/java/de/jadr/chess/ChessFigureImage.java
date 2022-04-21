package de.jadr.chess;

import java.awt.Image;
import java.awt.image.BufferedImage;
import java.io.BufferedInputStream;
import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;

import javax.imageio.ImageIO;

import de.jadr.chess.ChessFigure.Type;

public class ChessFigureImage {

	public static HashMap<Type, BufferedImage> white = new HashMap<>();
	public static HashMap<Type, BufferedImage> black = new HashMap<>();
	
	private static BufferedImage get(boolean white, Type t) throws IOException {
		String path = "chessFigures/"+
				(white?"white":"black")+"_"
				+t.toString().toLowerCase()
				+".png";
		InputStream stream = ChessFigureImage.class.getResourceAsStream(path);
		BufferedInputStream bin = new BufferedInputStream(stream);
		byte[] bytes = new byte[bin.available()];
		bin.read(bytes);
		BufferedImage bi = ImageIO.read(new ByteArrayInputStream(bytes));
		
		return bi;
	}
	
	public static BufferedImage getImage(boolean isWhite, Type t) {
		HashMap<Type, BufferedImage> h;
		if(isWhite) {
			h = white;
		}else {
			h = black;
		}
		BufferedImage i = h.get(t);
		if(i == null) {
			try {
				i = h.put(t, get(isWhite, t));
			} catch (IOException e) {
				e.printStackTrace();
			}
		}
		return i;
	}
	
}
