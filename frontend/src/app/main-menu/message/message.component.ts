//la ruta era sin barra, aquelarre.
import { Component, OnInit, OnDestroy, NgZone, inject } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { Subscription } from "rxjs";

import { MessageListenerService, MessageFormat } from "../message-listener/message-listener.service";
import { MessageHistoryService } from "../message-history/message-history.service";

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
  private subs = new Subscription();
  private channelId: string | null = "b9889189-6940-4176-943f-98384f7015e9";
  private ngZone = inject(NgZone); //Marcelo dice que esto no va a furular, hay que hacerlo como en friends (o sea con señaless)

  constructor(
    private messageListener: MessageListenerService,
    private messageHistory: MessageHistoryService
  ) {}

  ngOnInit() {
    if (this.channelId) this.initChat(this.channelId);
  }

  initChat(channelId: string) {
    if (!channelId) return;
    this.channelId = channelId;

    this.messageListener.init();

    // Cargar mensajes históricos
    this.subs.add(
      this.messageHistory.getMessages(channelId).subscribe({
        next: (msgs) => {
          this.ngZone.run(() => {
            this.messages = msgs.filter(msg => msg.type !== "ping"); //este filtro no termina de funcionar, hay que cambiarlo
          });
        },
        error: (err) => console.error("Error cargando histórico:", err)
      })
    );

    // Escuchar mensajes en tiempo real
    this.subs.add(
      this.messageListener.onMessage().subscribe(msg => {
        this.ngZone.run(() => {
          this.messages.push(msg);
        });
      })
    );

    // Socket abierto
    this.subs.add(
      this.messageListener.onOpen().subscribe(() => {
        console.log("Socket listo, ya se pueden enviar mensajes");
      })
    );
  }

  sendMessage() {
    const text = this.messageInput.trim();
    if (!text) return;

    // Obtener el user_id desde localStorage
    const user = JSON.parse(localStorage.getItem('user') || '{}');
    const senderId = user.user_id || "yo";

    // Mensaje que se enviará al servidor
    const msgToSend = { info: { text }, sender_id: senderId };

    this.messageListener.send(JSON.stringify(msgToSend));

    this.ngZone.run(() => {//ese run hace que Angular se de cuenta del cambio (teóricamente)
      this.messages.push({
        message: text,
        sender_id: senderId,
        type: "user"
      });
    });

    this.messageInput = "";
  }

  ngOnDestroy() {
    this.subs.unsubscribe();
  }
}
