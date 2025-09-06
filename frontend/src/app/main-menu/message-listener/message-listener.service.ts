import { Injectable } from "@angular/core";
import { WebSocketService } from "../../websocket-service/websocket-service";

const extension = "ws/message";

export interface MessageFormat {
  header: string;
  info: Record<string, string>;
}
@Injectable({ providedIn: 'root' })
export class MessageListenerService {
    private websocketService!: WebSocketService<MessageFormat>;

    private callbacks = {
        onOpen: () => console.log("Message connected"),
        onClose: (e: CloseEvent) => console.log(e),
        onMessage: (data: MessageFormat) => console.log(data),
        onError: (err: Event | Error) => console.error(err),
    };

    init(groupId: string) {
        this.websocketService = new WebSocketService<MessageFormat>(extension, this.callbacks);
        this.websocketService.setisMessage(true);    
        this.websocketService.setchannelId(groupId); 
        this.websocketService.connect();
    }
}


