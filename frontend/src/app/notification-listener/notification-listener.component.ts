import { Injectable } from "@angular/core";
import { WebSocketService } from "../websocket-service/websocket-service";

const extension = "/ws/notification";

export interface MessageFormat {
    header: string;
    info: Record<string, string>;
}

@Injectable({
    providedIn: "root",
})
export class NotificationListenerService {
    private websocketService: WebSocketService<MessageFormat>;

    //Example of callbacks
    private callbacks = {
        //TODO:@AlexGarciaPrada Implement a way to notify the user
        onOpen: () => console.log("Notification connected"),
        onClose: (e: CloseEvent) => console.log(e),
        onMessage: (data: MessageFormat) => console.log(data),
        onError: (err: Event | Error) => console.error(err),
    };

    constructor() {
        this.websocketService = new WebSocketService(extension, this.callbacks);
    }
}
