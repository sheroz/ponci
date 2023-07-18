/*

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.ServletOutputStream;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.*;
import java.net.URLEncoder;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.BasicFileAttributes;
import java.nio.file.attribute.FileTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Date;
import java.util.List;
import java.util.zip.GZIPOutputStream;

/**
 * @author Sheroz Khaydarov
 * @since 10.05.13 11:23
 */

public class HttpFileSender {

    public static final byte STREAM_DISPOSITION_AUTO = 0;
    public static final byte STREAM_DISPOSITION_INLINE = 1;
    public static final byte STREAM_DISPOSITION_ATTACHMENT = 2;

    private static final int DEFAULT_STREAM_BUFFER_SIZE = 8192; // 8 KB

    public static final String MULTIPART_BOUNDARY_SEPERATOR = "boundary-part-content";
    public static final String CONTENT_ENCODING_TYPE_GZIP = "gzip";
    public static final String CONTENT_TYPE_TEXT = "text";
    public static final String CONTENT_TYPE_IMAGE = "image";

    public static final String HEADER_CONTENT_RANGE = "Content-Range";
    public static final String HEADER_CONTENT_DISPOSITION = "Content-Disposition";
    public static final String HEADER_CONTENT_LENGTH = "Content-Length";
    public static final String HEADER_CONTENT_TYPE = "Content-Type";
    public static final String HEADER_ACCEPT_RANGES = "Accept-Ranges";
    public static final String HEADER_E_TAG = "ETag";
    public static final String HEADER_LAST_MODIFIED = "Last-Modified";
    public static final String HEADER_EXPIRES = "Expires";
    public static final String HEADER_ACCEPT = "Accept";
    public static final String HEADER_CONTENT_ENCODING = "Content-Encoding";
    public static final String HEADER_ACCEPT_ENCODING = "Accept-Encoding";
    public static final String HEADER_RANGE = "Range";
    public static final String HEADER_IF_RANGE = "If-Range";
    public static final String HEADER_IF_MATCH = "If-Match";
    public static final String HEADER_IF_NONE_MATCH = "If-None-Match";
    public static final String HEADER_IF_MODIFIED_SINCE = "If-Modified-Since";
    public static final String HEADER_IF_UNMODIFIED_SINCE = "If-Unmodified-Since";

    private Logger logger = LoggerFactory.getLogger(this.getClass().getName());

    private int streamBufferSize;
    private boolean supportGZIP;

    private class ByteRange {
        private long start;
        private long end;
        private long total;

        public ByteRange(long start, long end, long total) {
            this.start = start;
            this.end = end;
            this.total = total;
        }

        public long getLength() {
            return end - start + 1;
        }

        public boolean isFullRange() {
            return (total==getLength());
        }

        @Override
        public String toString() {
            return "ByteRange [start=" + start + ", end=" + end + ", length=" + getLength()
                    + ", total=" + total + ", full=" + isFullRange() + "]";
        }
    }


    public HttpFileSender() {
        streamBufferSize = DEFAULT_STREAM_BUFFER_SIZE;
        supportGZIP = false;
    }

    public void setStreamBufferSize(int streamBufferSize) {
        this.streamBufferSize = streamBufferSize;
    }

    public void setSupportGZIP(boolean supportGZIP) {
        this.supportGZIP = supportGZIP;
    }

    private List<ByteRange> validateRanges(HttpServletRequest request,
                               HttpServletResponse response,
                               long contentLength,
                               String eTag,
                               long lastModified
                               ) throws IOException {

        List<ByteRange> ranges = new ArrayList<ByteRange>();

        // Validate and process Range and If-Range headers.
        String range = request.getHeader(HEADER_RANGE);
        if (range != null) {

            // Range header should match format "bytes=n-n,n-n,n-n...". If not, then return 416.
            if (!range.matches("^bytes=\\d*-\\d*(,\\d*-\\d*)*$")) {
                response.setHeader(HEADER_CONTENT_RANGE, "bytes *_remove_space_/" + contentLength); // Required in 416.
                response.sendError(HttpServletResponse.SC_REQUESTED_RANGE_NOT_SATISFIABLE);
                return null;
            }

            // If-Range header should either match ETag or be greater then LastModified. If not,
            // then return full file.
            String ifRange = request.getHeader(HEADER_IF_RANGE);
            if (ifRange != null && !ifRange.equals(eTag)) {
                ByteRange full = new ByteRange(0, contentLength - 1, contentLength);
                try {
                    long ifRangeTime = request.getDateHeader(HEADER_IF_RANGE);
                    if (ifRangeTime != -1 && ifRangeTime + 1000 < lastModified) {
                        ranges.add(full);
                    }
                } catch (IllegalArgumentException ignore) {
                    ranges.add(full);
                }
            }

            // If any valid If-Range header, then process each part of byte range.
            if (ranges.isEmpty()) {
                for (String part : range.substring(6).split(",")) {
                    // Assuming a file with length of 100, the following examples returns bytes at:
                    // 50-80 (50 to 80), 40- (40 to length=100), -20 (length-20=80 to length=100).

                    long start = -1 ;
                    long end = -1;
                    String startDigits = part.substring(0, part.indexOf("-"));
                    String endDigits = part.substring(part.indexOf("-") + 1, part.length());
                    try {
                        start = (startDigits.length() > 0) ? Long.parseLong(startDigits) : -1;
                        end = (endDigits.length() > 0) ? Long.parseLong(endDigits) : -1;
                    } catch (NumberFormatException e) {
                        logger.error(e.toString());
                    }

                    if (start == -1) {
                        start = contentLength - end;
                        end = contentLength - 1;
                    } else if (end == -1 || end > contentLength - 1) {
                        end = contentLength - 1;
                    }

                    // Check if Range is syntactically valid. If not, then return 416.
                    if (start > end) {
                        response.setHeader(HEADER_CONTENT_RANGE, "bytes *_remove_space_/" + contentLength); // Required in 416.
                        response.sendError(HttpServletResponse.SC_REQUESTED_RANGE_NOT_SATISFIABLE);
                        return null;
                    }

                    ranges.add(new ByteRange(start, end, contentLength));
                }
            }
        }
        return ranges;
    }

    private String prepareDisposition(HttpServletRequest request, byte dispositionType, String contentType)
    {
        String disposition;
        disposition = "inline";
        switch (dispositionType) {
            case STREAM_DISPOSITION_AUTO:
                if (!contentType.startsWith(CONTENT_TYPE_TEXT) && !contentType.startsWith(CONTENT_TYPE_IMAGE)) {
                    String accept = request.getHeader(HEADER_ACCEPT);
                    if (!accepts(accept, contentType))
                        disposition = "attachment";
                }
                break;
            case STREAM_DISPOSITION_ATTACHMENT:
                disposition = "attachment";
                break;
        }
        return disposition;
    }


    private boolean validateForGZIP(HttpServletRequest request, String contentType)
    {
        boolean gzip = false;
        // If content type is text, then determine whether GZIP content encoding is supported by
        // the browser and expand content type with the one and right character encoding.
        if (contentType.startsWith(CONTENT_TYPE_TEXT)) {
            String acceptEncoding = request.getHeader(HEADER_ACCEPT_ENCODING);
            gzip = supportGZIP && accepts(acceptEncoding, CONTENT_ENCODING_TYPE_GZIP);
        }
        return gzip;
    }

    private HttpServletResponse prepareResponse(    HttpServletRequest request,
                                                    HttpServletResponse response,
                                                    String dispositionType,
                                                    String eTag,
                                                    String filename,
                                                    long lastModified) throws UnsupportedEncodingException {

        long cacheAge = 60 * 60 * 24 * 7 ; // 1 week, given in seconds
        long expiry = new Date().getTime() + cacheAge*1000; // in milliseconds

        response.reset();
        response.setBufferSize(streamBufferSize);

        String userAgent= request.getHeader("user-agent").toLowerCase();
        if(logger.isDebugEnabled())
            logger.debug("REQUEST HEADER: User-Agent: {}", userAgent);

        // works for: Firefox 21, Chrome 27, IE 9, Opera 12, ***except*** Safari (tested on Safari 5.1.7)
        if (userAgent.contains("safari"))
        {
            String encodedFilename = URLEncoder.encode(filename, "UTF-8");
            response.setHeader(HEADER_CONTENT_DISPOSITION, dispositionType + ";filename*=UTF-8''" + encodedFilename);
        } else
        {
            String encodedFilename = URLEncoder.encode(filename, "UTF-8");
            response.setHeader(HEADER_CONTENT_DISPOSITION, dispositionType + ";filename*=UTF-8''" + encodedFilename);
        }

        // cache options
        // response.addHeader("Cache-Control", "must-revalidate"); // optional
        // response.setHeader("Cache-Control", "max-age="+ cacheAge); // in seconds
        response.setHeader(HEADER_ACCEPT_RANGES, "bytes");
        response.setHeader(HEADER_E_TAG, eTag);
        response.setDateHeader(HEADER_LAST_MODIFIED, lastModified); // in milliseconds
        response.setDateHeader(HEADER_EXPIRES, expiry); // in milliseconds

        return response;
    }

    private boolean validateRequestHeads(HttpServletRequest request,
                                HttpServletResponse response,
                                String eTag,
                                long lastModified ) throws IOException {

        // Validate request headers for caching ---------------------------------------------------
        // If-None-Match header should contain "*" or ETag.
        String ifNoneMatch = request.getHeader(HEADER_IF_NONE_MATCH);
        if (ifNoneMatch != null && matches(ifNoneMatch, eTag)) {
            response.setHeader(HEADER_E_TAG, eTag);
            response.sendError(HttpServletResponse.SC_NOT_MODIFIED);
            return false;
        }

        // If-Modified-Since header should be greater than LastModified. If so, then return 304.
        // This header is ignored if any If-None-Match header is specified.
        long ifModifiedSince = request.getDateHeader(HEADER_IF_MODIFIED_SINCE);
        if (ifNoneMatch == null && ifModifiedSince != -1 && ifModifiedSince + 1000 > lastModified) {
            response.setHeader(HEADER_E_TAG, eTag);
            response.sendError(HttpServletResponse.SC_NOT_MODIFIED);
            return false;
        }

        // Validate request headers for resume ----------------------------------------------------
        // If-Match header should contain "*" or ETag. If not, then return 412.
        String ifMatch = request.getHeader(HEADER_IF_MATCH);
        if (ifMatch != null && !matches(ifMatch, eTag)) {
            response.sendError(HttpServletResponse.SC_PRECONDITION_FAILED);
            return false;
        }

        // If-Unmodified-Since header should be greater than LastModified. If not, then return 412.
        long ifUnmodifiedSince = request.getDateHeader(HEADER_IF_UNMODIFIED_SINCE);
        if (ifUnmodifiedSince != -1 && ifUnmodifiedSince + 1000 <= lastModified) {
            response.sendError(HttpServletResponse.SC_PRECONDITION_FAILED);
            return false;
        }

        return true;
    }


    public void sendFileBufferToHttpResponse(
            HttpServletRequest request,
            HttpServletResponse response,
            String contentType,
            long contentLength,
            long lastModified,
            byte dispositionType,
            String filename,
            byte [] sendArray) {

    }
    public void sendBufferToStream(OutputStream outStream, long start, long length, byte [] data)
            throws IOException
    {
        if (data !=null)
        {
            outStream.write(data);
        }
    }

    public void sendFileToHttpResponse( HttpServletRequest request,
                            HttpServletResponse response,
                            byte dispositionType,
                            String contentType,
                            String filename,
                            File file ) throws IOException {

        Path path = file.toPath();

        if (filename==null || filename.isEmpty())
            filename = path.getFileName().toString();

        BasicFileAttributes fileAttributes = Files.readAttributes(path, BasicFileAttributes.class);
        long fileLength = fileAttributes.size();

        FileTime fileTime = fileAttributes.lastModifiedTime();
        long lastModified = fileTime.toMillis();

        String eTag = makeETag(filename, fileLength, lastModified);

        OutputStream outStream = null;

        try {

            if (!validateRequestHeads(request, response, eTag, lastModified))
                return;

            List<ByteRange> ranges = validateRanges(request, response, fileLength, eTag, lastModified);
            if (logger.isDebugEnabled())
            {
                logger.debug(this.getClass().getName()+" - validated ranges count: {}", ((ranges!=null) ? ranges.size() : null));
                if (ranges!=null)
                {
                    for (ByteRange byteRange: ranges)
                        logger.debug(byteRange.toString());
                }
            }
            if (ranges==null)
                return;

            if (contentType == null)
                contentType = "application/octet-stream";

            String disposition  = prepareDisposition(request, dispositionType, contentType);

            boolean gzip = validateForGZIP(request, contentType);

            if (gzip)
                contentType += ";charset=UTF-8";

            response = prepareResponse(request, response, disposition, eTag, filename, lastModified);

            String method = request.getMethod();
            boolean sendResponseBody = method.equalsIgnoreCase("GET");

            outStream = response.getOutputStream();

            if (ranges.isEmpty() || ranges.size() == 1)
            {
                response.setContentType(contentType);
                if (ranges.isEmpty() || ranges.get(0).isFullRange())
                {
                    // full range
                    if (!gzip)
                        response.setHeader(HEADER_CONTENT_LENGTH, String.valueOf(fileLength));

                    if (sendResponseBody)
                    {
                        if (gzip)
                        {
                            response.setHeader(HEADER_CONTENT_ENCODING, CONTENT_ENCODING_TYPE_GZIP);
                            outStream = new GZIPOutputStream(outStream, streamBufferSize);
                        }
                        sendFileToStream(outStream, 0, fileLength, file);
                    }

                } else
                {
                    // partial range
                    ByteRange byteRange = ranges.get(0);
                    response.setHeader(HEADER_CONTENT_RANGE, "bytes " + byteRange.start + "-" + byteRange.end + "/" + byteRange.total);
                    response.setHeader(HEADER_CONTENT_LENGTH, String.valueOf(byteRange.getLength()));
                    response.setStatus(HttpServletResponse.SC_PARTIAL_CONTENT);
                    if (sendResponseBody)
                        sendFileToStream(outStream, byteRange.start, byteRange.getLength(), file);
                }
            } else {
                // multipart range
                response.setContentType("multipart/byteranges; boundary=" + MULTIPART_BOUNDARY_SEPERATOR);
                response.setStatus(HttpServletResponse.SC_PARTIAL_CONTENT);
                if (sendResponseBody) {
                    // process each byte range
                    ServletOutputStream servletOutputStream = (ServletOutputStream) outStream;
                    for (ByteRange byteRange : ranges) {
                        servletOutputStream.println();
                        servletOutputStream.println("--" + MULTIPART_BOUNDARY_SEPERATOR);
                        servletOutputStream.println(HEADER_CONTENT_TYPE + ": " + contentType);
                        servletOutputStream.println(HEADER_CONTENT_RANGE + ": bytes " + byteRange.start + "-" + byteRange.end + "/" + byteRange.total);
                        sendFileToStream(servletOutputStream, byteRange.start, byteRange.getLength(), file);
                    }
                    servletOutputStream.println();
                    servletOutputStream.println("--" + MULTIPART_BOUNDARY_SEPERATOR + "--");
                }
            }

        } catch (Exception e) {
            logger.error(getClass().getName()+": " + e.toString());
            response.sendError(HttpServletResponse.SC_NOT_FOUND);
        } finally {
            if (outStream!=null)
                outStream.close();
        }
    }

    public void sendFileToHttpResponse(HttpServletRequest request,
                                       HttpServletResponse response,
                                       String contentType,
                                       byte dispositionType,
                                       String filename,
                                       String fileFullPath
                                       ) throws IOException {

            File file;
            file = openInputFile(fileFullPath, response);
            if (file==null)
                return;

            sendFileToHttpResponse( request, response, dispositionType, contentType, filename, file);
    }

    /**
     * Send given byte range of the given FileStorage object to the given output.
     * @param outStream The OutputStream to copy the given range.
     * @param start Start of the byte range.
     * @param length Length of the byte range.
     * @param file input File instance.
     * @throws IOException If something fails at I/O level.
     */
    public void sendFileToStream(OutputStream outStream, long start, long length, File file)
           throws IOException
    {
/*
        // File streams - too slow !!!
        InputStream inputStream = new FileInputStream(file);
        int bytesRead;
        byte[] buffer = new byte[streamBufferSize];
        while ((bytesRead = inputStream.read(buffer)) != -1) {
            outStream.write(buffer, 0, bytesRead);
        }
        inputStream.close();
*/
        // RandomAccessFile method is 5 times faster than streaming !!!
        if (logger.isDebugEnabled())
            logger.debug("File download request: File = {}, start position ={}, bytes to send = {}", file.getAbsolutePath(), start, length);

        RandomAccessFile storageFile;
        storageFile = new RandomAccessFile(file, "r");
        storageFile.seek(start);

        int bytesRead;
        byte[] buffer = new byte[streamBufferSize];

        int bytesToSend;
        long sentCount=0;

        try {
            while ((bytesRead = storageFile.read(buffer)) != -1)
            {
                if (bytesRead > (length-sentCount))
                    bytesToSend = (int) (length-sentCount);
                else
                    bytesToSend = bytesRead;

                outStream.write(buffer, 0, bytesToSend);
                sentCount += bytesToSend;
            }
        } catch (IOException e) {
            // mostly throws network related exceptions, such as
            // ClientAbortException, broken pipe and etc,
            // simply discard to avoid misleads in logs ))
            if (logger.isDebugEnabled())
                logger.debug(e.toString());
        }

        storageFile.close();
        if (logger.isDebugEnabled())
            logger.debug("File send completed: File = {}, start position ={}, bytes to send = {}, sent bytes = {}", file.getAbsolutePath(), start, length, sentCount);

/*
        // Mapping a file into memory - the fastest method to serve "hot" files
        // Problem : There is no safe and documented way to unmap memory and free allocated file resources.
        // Would watch progress on future JDK and implement later

        RandomAccessFile randomAccessFile = new RandomAccessFile(fullname, "r");
        FileChannel fileChannel = randomAccessFile.getChannel();
        MappedByteBuffer mappedByteBuffer = fileChannel.map(FileChannel.MapMode.READ_ONLY, 0, fileChannel.size());

        fileChannel.close();
        randomAccessFile.close();
*/
    }

    public void sendFileToHttpResponse(HttpServletRequest request,
                                       HttpServletResponse response,
                                       byte dispositionType,
                                       String fileFullPath ) throws IOException
    {

        File file;
        file = openInputFile(fileFullPath, response);
        if (file==null)
            return;

        String filename = file.getName();
        String contentType =  request.getServletContext().getMimeType(filename);

        sendFileToHttpResponse( request, response, dispositionType, contentType, filename, file);
    }

    private File openInputFile(String fileFullPath, HttpServletResponse response)
            throws IOException
    {

        if (fileFullPath==null || fileFullPath.isEmpty())
        {
            response.sendError(HttpServletResponse.SC_NOT_FOUND);
            return null;
        }

        File file = new File(fileFullPath);

        if (!file.exists())
        {
            response.sendError(HttpServletResponse.SC_NOT_FOUND);
            return null;
        }

        if(!file.canRead())
        {
            response.sendError(HttpServletResponse.SC_FORBIDDEN);
            return null;
        }

        if(!file.isFile())
        {
            response.sendError(HttpServletResponse.SC_NOT_FOUND);
            return null;
        }

        return file ;
    }

    /**
     * Returns ETag identification
     * @param filename The filename value.
     * @param length The file length value .
     * @param time The file last modification time.
     * @return ETag value.
     */
    public static String makeETag(String filename, long length, long time)
    {
        return filename + "_" + length + "_" + time;
    }

    /**
     * Returns true if the given match header matches the given value.
     * @param matchHeader The match header.
     * @param toMatch The value to be matched.
     * @return True if the given match header matches the given value.
     */
    private static boolean matches(String matchHeader, String toMatch)
    {
        String[] matchValues = matchHeader.split("\\s*,\\s*");
        Arrays.sort(matchValues);
        return Arrays.binarySearch(matchValues, toMatch) > -1
                || Arrays.binarySearch(matchValues, "*") > -1;
    }

    /**
     * Returns true if the given accept header accepts the given value.
     * @param acceptHeader The accept header.
     * @param toAccept The value to be accepted.
     * @return True if the given accept header accepts the given value.
     */
    private static boolean accepts(String acceptHeader, String toAccept)
    {
        boolean accepts = false;
        if (acceptHeader!=null && toAccept!=null)
        {
            String[] acceptValues = acceptHeader.split("\\s*(,|;)\\s*");
            Arrays.sort(acceptValues);
            accepts = Arrays.binarySearch(acceptValues, toAccept) > -1
                || Arrays.binarySearch(acceptValues, toAccept.replaceAll("/.*$", "/*")) > -1
                || Arrays.binarySearch(acceptValues, "*/*") > -1;
        }
        return accepts;
    }

}

*/