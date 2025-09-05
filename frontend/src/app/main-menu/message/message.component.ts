import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { SERVER_ROUTE } from "../../../environment/environment.secret";

@Component({
  selector: "app-message",
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: "./message.component.html",
  styleUrls: ["./message.component.scss"],
})
export class MessageComponent {
  ws!: WebSocket;
  messages: { user: string; text: string }[] = [];
  inputText = "";

  ngOnInit() {
    const wsUrl = SERVER_ROUTE.replace(/^http/, "ws") + "ws"; //preguntar a Alex porque cojones hay que usar ese ws y no https

    this.ws = new WebSocket(wsUrl);

    this.ws.onopen = () => {
      console.log("¡Pez! WS conectado a", wsUrl);
    };

    this.ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data);
        this.messages.push({
          user: msg.user || "anon",
          text: msg.text || JSON.stringify(msg),
        });
      } catch (e) {
        console.error("Mensaje WS inválido:", event.data);
      }
    };

    this.ws.onerror = (err) => {
      console.error("WS error", err);
    };
  }

  sendMessage() {
    if (!this.inputText.trim()) return;
//CHAPUZOTE LEGENDARIO, SOLO ES PARA VER QUE FUNCIONA MÍNIMAMENTE 
    const prueba = {
      user: "yo",        // más adelante irá el user real
      text: this.inputText,
      groupId: "demo",   // más adelante bubble.id
    };

    this.ws.send(JSON.stringify(prueba));
    this.inputText = "";
  }
}
