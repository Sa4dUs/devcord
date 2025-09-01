import { Injectable } from "@angular/core";
import { WebSocketService } from "../websocket/websocket";

const extension = "/ws/notification";

interface MessageFormat {
    header: string;
    info: Record<string, string>;
}
@Injectable({
    providedIn: "root",
})
export class NotificationListenerService extends WebSocketService<MessageFormat> {
    constructor() {
        super(extension);
    }
}
