package de.jadr;

import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.net.URI;
import java.net.URL;
import java.net.URLDecoder;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.UUID;

import de.jadr.webmodules.Request;
import de.jadr.webserver.DirectoryManager.FileData;
import de.jadr.webserver.JadrHTTPServer;
import de.jadr.webserver.JadrHTTPServer.HTTPHandler;
import utils.HTTPUtils;

public class ChessWeb {

    public static Map<String, String> splitQuery(String query) throws UnsupportedEncodingException {
        Map<String, String> query_pairs = new LinkedHashMap<String, String>();
        if(query == null) return query_pairs;
        String[] pairs = query.split("&");
        for (String pair : pairs) {
            int idx = pair.indexOf("=");
            if(idx==-1)continue;
            query_pairs.put(URLDecoder.decode(pair.substring(0, idx), "UTF-8"), URLDecoder.decode(pair.substring(idx + 1), "UTF-8"));
        }
        return query_pairs;
    }

    public static void main(String[] args) throws IOException {
        final JadrHTTPServer jahhttp = new JadrHTTPServer(7720);
        final ChessWebSocketServer cwss = new ChessWebSocketServer(7721);
        cwss.start();
        jahhttp.start();
        jahhttp.addDefaultWebServer(new HTTPHandler() {

            public FileData handle(Request request) throws Exception {
                Map<String, String> map = splitQuery(request.getHttpExchange().getRequestURI().getQuery());
                if(map.containsKey("lobbyid")) {
                    UUID uuid = UUID.fromString(map.get("lobbyid"));
                    ChessLobby cl = cwss.lobbys.get(uuid);

                    if(cl == null) {
                        FileData fd = new FileData();
                        fd.mime = HTTPUtils.getMime("html");
                        fd.bytes = "<h1>Lobby not found</h1>".getBytes();
                        return fd;
                    }
                }
                return null;
            }
        });


    }

}