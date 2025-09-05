import { Component, Injectable } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { WebSocketService } from "../../websocket-service/websocket-service";
import { MessageFormat } from "../../notification-listener/notification-listener.component";
const extension="/ws/message";

export interface MessageComponent {
    header: string;
    info: Record<string, string>;
  }
@Injectable({
  providedIn: "root",
})



export class MessageListenerService{
  private websocketService: WebSocketService<MessageFormat>;

private callbacks={
  
} 
constructor (){
  this.websocketService=new WebSocketService(extension, this.callbacks);
}
}
