import { Injectable } from "@angular/core";
import { Subject, Observable } from "rxjs";
import { WebSocketService } from "../../websocket-service/websocket-service";

export interface MessageFormat  {
  id?: string;
  sender_id?: string;
  channel_id?: string;
  info?: Record<string, string>; 
  created_at?: string;
  message?: string;//hubo que añadirlo por cojones 
  type?:string;
}

@Injectable({ providedIn: 'root' })
export class MessageListenerService {
  private ws!: WebSocketService<MessageFormat>;
  private messageSubject = new Subject<MessageFormat>();
  private openSubject = new Subject<void>();
  private closeSubject = new Subject<void>();

  private channelId = "b9889189-6940-4176-943f-98384f7015e9"; // está a fuego, habrá que cambiarlo en algún momento

  private isMessage = true; 

//Para Marcelo, seguro que pregunta para que los observable, contestar
  public onMessage(): Observable<MessageFormat> {
    return this.messageSubject.asObservable();
  }

  public onOpen(): Observable<void> {
    return this.openSubject.asObservable();
  }

  public onClose(): Observable<void> {
    return this.closeSubject.asObservable();
  }

  init() {
    this.ws = new WebSocketService<MessageFormat>( //no he sido capaz de eliminar el new
      //(mentira, no se puede porque sino nunca creas una instancia, que tb he transitado por esa fase--_--)
      "ws/message", //mas rutas a fuego, pero a esta se le tiene cariño
      {
        onOpen: () => {
          console.log("WebSocket abierto correctamente");
          this.openSubject.next();
        },
        onClose: (e) => {
          console.warn("WebSocket cerrado", e);
          this.closeSubject.next();
        },
        onMessage: (msg) => {
          this.messageSubject.next(msg);
          console.log("Mensaje recibido:", msg);
        },
        onError: (err) => {
          console.error("MessageListener WS error:", err);
        },
      },
      {
        isMessage: this.isMessage,
        channelId: this.channelId
      }
    );

    }

  send(text: string) {
    if (!text) return;
    const payload: MessageFormat = { info: { text } };
    this.ws.send(payload);
  }

  close() {
    this.ws?.close();
  }
}
