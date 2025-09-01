import { Observable, Subject } from "rxjs";
import { SERVER_ROUTE } from "../../environment/environment.secret";

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
    private messageSubject = new Subject<T>();
    private socket!: WebSocket;
    private extension: string;

    private onOpen?: OpenCallback;
    private onClose?: CloseCallback;
    private onMessage?: MessageCallback<T>;
    private onError?: ErrorCallback;

    constructor(extension: string, callbacks?: WebSocketCallbacks<T>) {
        this.extension = extension;

        this.onOpen = callbacks?.onOpen;
        this.onClose = callbacks?.onClose;
        this.onMessage = callbacks?.onMessage;
        this.onError = callbacks?.onError;

        this.connectWebSocket();
    }

    public connect(): Observable<T> {
        this.connectWebSocket();
        return this.messageSubject.asObservable();
    }

    private connectWebSocket() {
        const token = localStorage.getItem("token");
        if (!token) return;

        const wsUrl = SERVER_ROUTE + this.extension;
        this.socket = new WebSocket(wsUrl, [token]);

        this.socket.onopen = () => {
            //This is just for debugging
            console.log("WS opened: " + this.extension);
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
            if (this.onClose) this.onClose(event);
            //Just until we discover how to keep it alive
            this.connectWebSocket();
        };

        this.socket.onerror = (err) => {
            console.error("WS error: " + this.extension, err);
            if (this.onError) this.onError(err);
        };
    }
}
