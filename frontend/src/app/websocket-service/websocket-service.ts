import { SERVER_ROUTE } from "../../environment/environment.secret";

type OpenCallback = () => void;                     
type CloseCallback = (event: CloseEvent) => void;   
type MessageCallback<T> = (message: T) => void;    
type ErrorCallback = (error: Event | Error) => void; 

//esto antes estaba hecho una pelota unilinea con lo de arriba, por lo visto lo suyo es separarlo.
interface WebSocketCallbacks<T> {
    onOpen?: OpenCallback;
    onClose?: CloseCallback;
    onMessage?: MessageCallback<T>;
    onError?: ErrorCallback;
}
export class WebSocketService<T> {
    private socket!: WebSocket; 
    private extension: string;  
    private keepAliveInterval: ReturnType<typeof setInterval> | null = null; 
    private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;  
    private reconnectAttempts = 0; 

    // Callbacks locales
    private onOpen?: OpenCallback;
    private onClose?: CloseCallback;
    private onMessage?: MessageCallback<T>;
    private onError?: ErrorCallback;

    private isMessage: boolean; 
    private channelId?: string; 

    constructor(
        extension: string, 
        callbacks?: WebSocketCallbacks<T>, 
        options?: { isMessage?: boolean, channelId?: string }
    ) {
        this.extension = extension;
        this.onOpen = callbacks?.onOpen;
        this.onClose = callbacks?.onClose;
        this.onMessage = callbacks?.onMessage;
        this.onError = callbacks?.onError;
        this.isMessage = options?.isMessage ?? false;
        this.channelId = options?.channelId;
        //aquí se crea la instancia nueva
        this.connectWebSocket();
    }
//la paranoia de los JSON.stringify viene de lo que dijo Jorge, no he quitado la interfaz y ahora hay que separarlo 
    private startKeepAlive() {
        this.keepAliveInterval = setInterval(() => {
            if (this.socket.readyState === WebSocket.OPEN) {
                this.socket.send(JSON.stringify({ type: "ping" }));
            }
        }, 30000);
    }

    private stopKeepAlive() {
        if (this.keepAliveInterval) clearInterval(this.keepAliveInterval);
    }

    private reconnectWithBackoff() {
        if (this.reconnectTimeout) clearTimeout(this.reconnectTimeout);

        const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
        console.log("Reconnecting in " + delay + "ms");

        this.reconnectTimeout = setTimeout(() => {
            this.reconnectAttempts++;
            this.connectWebSocket(); 
        }, delay);
    }

    private connectWebSocket() {
        const token = localStorage.getItem("token"); 
        if (!token) return; 

        console.log("Token:", token);
        console.log("channelId:", this.channelId);

        const wsUrl = SERVER_ROUTE+"/" + this.extension;
        console.log("Esta es la ruta", wsUrl);

        if (this.isMessage && this.channelId) {
            this.socket = new WebSocket(wsUrl, [token, this.channelId]);
        } else {
            this.socket = new WebSocket(wsUrl, [token]);
        }

        this.socket.onopen = () => {
            console.log("WS opened: " + this.extension);
            this.startKeepAlive(); 
            if (this.onOpen) this.onOpen();
        };


        this.socket.onmessage = (event) => {
            try {
                const data: T = JSON.parse(event.data);
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

        // Evento de error
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
            console.warn("WebSocket not ready, message not sent:", msg);
        }
    }
//cierre manual
    public close() {
        this.socket?.close();
    }
}
