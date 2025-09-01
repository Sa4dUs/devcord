import { Observable, Subject } from "rxjs";
import { SERVER_ROUTE } from "../../environment/environment.secret";

//T is the type of the "messages" that are exchanged

export abstract class WebSocketService<T> {
    private messageSubject = new Subject<T>();
    private socket!: WebSocket;
    private extension: string;

    constructor(extension: string) {
        this.extension = extension;
        this.connect();
    }

    public connect(): Observable<T> {
        this.connectWebSocket();
        return this.messageSubject.asObservable();
    }

    private connectWebSocket() {
        const token = localStorage.getItem("token");

        if (!token) {
            return;
        }

        const wsUrl = SERVER_ROUTE + this.extension;

        this.socket = new WebSocket(wsUrl, [token]);

        this.socket.onopen = () => {
            //Just for debugging
            console.log("WS opened: " + this.extension);
        };

        this.socket.onmessage = (event) => {
            console.log(event.data);
            try {
                const data = JSON.parse(event.data);
                this.messageSubject.next(data);
            } catch (err) {
                console.error("Error parsing JSON:", err);
            }
        };

        this.socket.onclose = (event) => {
            console.log("WS closed: " + this.extension, event);
            //TODO:@AlexGarciaPrada this is a shitty solution
            this.connectWebSocket();
        };

        this.socket.onerror = (err) => {
            console.error("WS error: " + this.extension, err);
        };
    }
}
