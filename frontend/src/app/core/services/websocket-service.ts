import { Observable, Subject } from "rxjs";
import { SERVER_ROUTE } from "../../../environment/environment.secret";

/**
 * WebSocketService<T>
 *
 * Abstract base class for connecting to a WebSocket server and handling typed messages
 * using RxJS Observables.
 *
 * T: The type of the messages exchanged over the WebSocket.
 *
 * Features:
 * - Automatic connection to the WebSocket server with the given extension.
 * - Parses incoming JSON messages and emits them via a Subject as an Observable<T>.
 * - Handles WebSocket events: onopen, onmessage, onclose (with automatic reconnection), and onerror.
 *
 * Usage:
 * Extend this class and provide the message type T, then pass the WebSocket extension
 * to the constructor of the base class.
 */

type OpenCallback = () => void;
type CloseCallback = (event: CloseEvent) => void;
type MessageCallback<T> = (message: T) => void;
type ErrorCallback = (error: Event | Error) => void;

interface WebSocketCallbacks<T> {
    onOpen?: OpenCallback;
    onClose?: CloseCallback;
    onMessage?: MessageCallback<T>;
    onError?: ErrorCallback;
}

export class WebSocketService<T> {
    private readonly messageSubject = new Subject<T>();
    private socket!: WebSocket;
    private readonly extension: string;

    private keepAliveInterval: ReturnType<typeof setInterval> | null = null;
    private readonly keepAliveTime = 30000;

    private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
    private reconnectAttempts = 0;

    private readonly onOpen?: OpenCallback;
    private readonly onClose?: CloseCallback;
    private readonly onMessage?: MessageCallback<T>;
    private readonly onError?: ErrorCallback;

    constructor(
        extension: string,
        callbacks?: WebSocketCallbacks<T>,
        protocols?: string | string[],
    ) {
        this.extension = extension;

        this.onOpen = callbacks?.onOpen;
        this.onClose = callbacks?.onClose;
        this.onMessage = callbacks?.onMessage;
        this.onError = callbacks?.onError;

        this.connectWebSocket(protocols);
    }

    private startKeepAlive() {
        this.keepAliveInterval = setInterval(() => {
            if (this.socket.readyState === WebSocket.OPEN)
                this.socket.send(JSON.stringify({ type: "ping" }));
        }, this.keepAliveTime);
    }

    private stopKeepAlive() {
        if (this.keepAliveInterval) clearInterval(this.keepAliveInterval);
    }

    private reconnectWithBackoff() {
        if (this.reconnectTimeout) clearTimeout(this.reconnectTimeout);

        const delay = Math.min(
            1000 * Math.pow(2, this.reconnectAttempts),
            30000,
        );
        console.log("Reconnecting in " + delay + "ms");

        this.reconnectTimeout = setTimeout(() => {
            this.reconnectAttempts++;
            this.connectWebSocket();
        }, delay);
    }

    public connect(): Observable<T> {
        this.connectWebSocket();
        return this.messageSubject.asObservable();
    }

    private connectWebSocket(protocols?: string | string[]) {
        const wsUrl = SERVER_ROUTE + this.extension;
        this.socket = new WebSocket(wsUrl, protocols);

        this.socket.onopen = () => {
            //This is just for debugging
            console.log("WS opened: " + this.extension);
            this.startKeepAlive();

            if (this.onOpen) this.onOpen();
        };

        this.socket.onmessage = (event) => {
            try {
                const data: T = JSON.parse(event.data);
                this.messageSubject.next(data);
                if (this.onMessage) this.onMessage(data);
            } catch (err) {
                console.error("Error parsing JSON:", err);
                if (this.onError) this.onError(err as Error);
            }
        };

        this.socket.onclose = (event) => {
            console.log("WS closed: " + this.extension, event);
            this.stopKeepAlive();

            if (this.onClose) this.onClose(event);
            this.reconnectWithBackoff();
        };

        this.socket.onerror = (err) => {
            console.error("WS error: " + this.extension, err);
            this.stopKeepAlive();

            if (this.onError) this.onError(err);
        };
    }
    public send(msg: T) {
        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify(msg));
        } else {
            console.warn("WebSocket not ready, message queued:", msg);
        }
    }
}
