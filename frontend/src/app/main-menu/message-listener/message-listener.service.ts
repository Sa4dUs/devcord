import { Injectable } from "@angular/core";
import { Subject, Observable } from "rxjs";

export interface MessageFormat {
  header: string;
  info: Record<string, string>;
}

@Injectable({ providedIn: 'root' })
export class MessageListenerService {
  private websocket!: WebSocket;
  private messageSubject = new Subject<MessageFormat>();
  private openSubject = new Subject<void>();
  private closeSubject = new Subject<void>();

  public onMessage(): Observable<MessageFormat> {
    return this.messageSubject.asObservable();
  }

  public onOpen(): Observable<void> {
    return this.openSubject.asObservable();
  }

  public onClose(): Observable<void> {
    return this.closeSubject.asObservable();
  }

  // Ahora recibe channelId como parámetro
  init(channelId: string) {
    const token = localStorage.getItem("token");
    if (!token) {
      console.error("No hay token, login requerido");
      return;
    }

    const url = `wss://lamoara.duckdns.org/devcord/ws/message`;
    console.log("Estoy aquí");
    console.log("primero token", token, "channelId",channelId);
    this.websocket = new WebSocket(url, [token, channelId]);

    this.websocket.onopen = () => {
      console.log("Conectado al WebSocket de mensajes");
      this.openSubject.next();
    };

    this.websocket.onclose = (e) => {
      console.log("WebSocket cerrado", e);
      this.closeSubject.next();
    };

    this.websocket.onerror = (err) => {
      console.error("Error WebSocket:", err);
    };

    this.websocket.onmessage = (event) => {
      try {
        const msg: MessageFormat = JSON.parse(event.data);
        this.messageSubject.next(msg);
        console.log("Mensaje recibido:", msg);
      } catch (e) {
        console.error("Error en mensaje WebSocket:", e);
      }
    };
  }

  send(text: string) {
    if (!this.websocket || this.websocket.readyState !== WebSocket.OPEN) {
      console.error("WebSocket no está abierto");
      return;
    }

    const payload: MessageFormat = {
      header: "message",
      info: { text }
    };

    this.websocket.send(JSON.stringify(payload));
  }

  close() {
    this.websocket?.close();
  }
}
