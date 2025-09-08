import { Component, OnInit, OnDestroy } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { MessageListenerService, MessageFormat } from "../message-listener/message-listener.service";
import { MessageHistoryService } from "../message-history/message-history.service";
import { Subscription } from "rxjs";

@Component({
  selector: "app-message",
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: "./message.component.html",
  styleUrls: ["./message.component.scss"],
})
export class MessageComponent implements OnInit, OnDestroy {
  messageInput = "";
  messages: MessageFormat[] = [];
  private messageSub?: Subscription;

  private channelId: string | null = null;

  constructor(
    private messageListener: MessageListenerService,
    private messageHistory: MessageHistoryService
  ) {}

  ngOnInit() {
  }

  initChat(channelId: string) {
    this.channelId = channelId;

    this.messageListener.init(channelId);

    this.messageSub = this.messageListener.onMessage().subscribe(msg => {
      this.messages.push(msg);
    });

    this.messageListener.onOpen().subscribe(() => {
      console.log("Socket listo, ya se pueden enviar mensajes");
    });
  }

  sendMessage() {
    const text = this.messageInput.trim();
    console.log("Enviando chorizo");

    this.messageListener.send(text);

    // feedback en pantalla
    this.messages.push({
      header: "message",
      info: { user: "Yo", text }
    });

    this.messageInput = "";
  }

  ngOnDestroy() {
    this.messageSub?.unsubscribe();
    this.messageListener.close();
  }
}
