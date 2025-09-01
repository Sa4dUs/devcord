import { Component, ElementRef, ViewChild } from "@angular/core";
import { WebSocketService } from "../../websocket-service/websocket-service";

//Fill it with the actual extension
const extension = "ws://192.168.1.145:3002";

export enum WebSocketMessageType {
    ConnectToRoom = "connecttoroom",
    RTCAnswer = "rtcanswer",
    RTCCandidate = "rtccandidate",
    Offer = "offer",
    Answer = "answer",
    Candidate = "candidate",
}

type MessageFormat =
    | { type: WebSocketMessageType.ConnectToRoom; room_id: string }
    | {
          type: WebSocketMessageType.RTCCandidate;
          candidate: RTCIceCandidateInit;
      }
    | {
          type: WebSocketMessageType.RTCAnswer;
          answer: RTCSessionDescriptionInit;
      }
    | { type: WebSocketMessageType.Offer; sdp: string }
    | { type: WebSocketMessageType.Answer; sdp: string }
    | { type: WebSocketMessageType.Candidate; candidate: RTCIceCandidateInit };

@Component({
    selector: "call.component",
    imports: [],
    templateUrl: "./call.component.html",
    styleUrl: "./call.component.scss",
})
export class CallComponent {
    @ViewChild("localVideo", { static: true })
    localVideoRef!: ElementRef<HTMLVideoElement>;
    @ViewChild("remoteMediaContainer", { static: true })
    remoteMediaContainerRef!: ElementRef<HTMLDivElement>;

    private websocketService: WebSocketService<MessageFormat>;

    //This is a way I found to do it (the function were too large (eso dijo ella))
    private callbacks = {
        onOpen: this.handleOpen.bind(this),
        onMessage: this.handleMessage.bind(this),
    };

    private localStream!: MediaStream;
    private peerConnection!: RTCPeerConnection;
    private remoteStreams = new Map<string, MediaStream>();

    private configuration: RTCConfiguration = {
        iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
    };

    constructor() {
        this.websocketService = new WebSocketService(extension, this.callbacks);
    }

    async handleOpen() {
        this.websocketService.send({
            type: WebSocketMessageType.ConnectToRoom,
            room_id: "A",
        });
        // eslint-disable-next-line no-undef
        this.localStream = await navigator.mediaDevices.getUserMedia({
            video: false,
            audio: true,
        });

        this.localVideoRef.nativeElement.srcObject = this.localStream;

        console.log("Total tracks: ", this.localStream.getTracks().length);

        this.peerConnection = new RTCPeerConnection(this.configuration);

        this.localStream.getTracks().forEach((track, index) => {
            console.log(`Track #${index}: kind=${track.kind}, id=${track.id}`);
            this.peerConnection.addTrack(track, this.localStream);
        });

        let counter = 0;
        this.peerConnection.ontrack = (event) => {
            if (counter < 2) {
                counter += 1;
                return;
            }

            const [stream] = event.streams;
            const userId = stream.id;

            if (!this.remoteStreams.has(userId)) {
                console.log("Nuevo usuario remoto:", userId);
                this.remoteStreams.set(userId, stream);

                const container = document.createElement("div");
                container.className = "user-media";
                container.id = `remote-user-${userId}`;

                const video = document.createElement("video");
                video.autoplay = true;
                video.playsInline = true;
                video.srcObject = stream;
                video.load();

                video.onloadedmetadata = () => {
                    video
                        .play()
                        .catch((err) => console.warn("Video play error:", err));
                };

                const label = document.createElement("div");
                label.textContent = `Usuario: ${userId}`;

                container.appendChild(label);
                container.appendChild(video);
                this.remoteMediaContainerRef.nativeElement.appendChild(
                    container,
                );
            } else {
                console.log("Stream adicional para usuario existente:", userId);
            }
        };

        this.peerConnection.onicecandidate = (event) => {
            if (event.candidate) {
                this.websocketService.send({
                    type: WebSocketMessageType.RTCCandidate,
                    candidate: event.candidate,
                });
            }
        };

        this.websocketService.send({
            type: WebSocketMessageType.ConnectToRoom,
            room_id: "A",
        });
    }

    async handleMessage(data: MessageFormat) {
        console.log("WS raw message:", data);

        if (data.type === WebSocketMessageType.Offer) {
            await this.peerConnection.setRemoteDescription(
                new RTCSessionDescription(data),
            );
            const answer = await this.peerConnection.createAnswer();
            await this.peerConnection.setLocalDescription(answer);
            this.websocketService.send({
                type: WebSocketMessageType.RTCAnswer,
                answer: answer,
            });
        } else if (data.type === WebSocketMessageType.Answer) {
            //Maybe is neccesary to handle
        } else if (data.type === WebSocketMessageType.Candidate) {
            await this.peerConnection.addIceCandidate(
                new RTCIceCandidate(data.candidate),
            );
        }
    }

    handleClose() {
        if (this.peerConnection) this.peerConnection.close();
        if (this.localStream)
            this.localStream.getTracks().forEach((track) => track.stop());
        this.localVideoRef.nativeElement.srcObject = null;

        for (const [id] of this.remoteStreams) {
            const el = document.getElementById(`remote-user-${id}`);
            if (el) el.remove();
        }

        this.remoteStreams.clear();
    }
}
