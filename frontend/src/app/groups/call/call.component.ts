import { Component, ElementRef, ViewChild } from "@angular/core";
import { WebSocketService } from "../../websocket-service/websocket-service";
import { ActivatedRoute } from "@angular/router";

//Fill it with the actual extension
const extension = "/ws/call";

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
    selector: "call",
    imports: [],
    templateUrl: "./call.component.html",
    styleUrl: "./call.component.scss",
})
export class CallComponent {
    @ViewChild("localVideo", { static: true })
    localVideoRef!: ElementRef<HTMLVideoElement>;
    @ViewChild("remoteMediaContainer", { static: true })
    remoteMediaContainerRef!: ElementRef<HTMLDivElement>;

    private groupId = "";

    private websocketService: WebSocketService<MessageFormat>;

    //This is a way I found to do it (the function were too large (eso dijo ella))
    private callbacks = {
        onOpen: this.handleOpen.bind(this),
        onMessage: this.handleMessage.bind(this),
    };

    //For getting the id, neccesary for making the room
    ngOnInit() {
        this.route.params.subscribe((params) => {
            this.groupId = params["groupId"];
            console.log("GroupId:", this.groupId);
        });
    }
    private localStream!: MediaStream;
    private peerConnection!: RTCPeerConnection;
    private remoteStreams = new Map<string, MediaStream>();

    public cameraOn = true;
    public micOn = true;

    private configuration: RTCConfiguration = {
        iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
    };

    constructor(private route: ActivatedRoute) {
        this.websocketService = new WebSocketService(extension, this.callbacks);
    }

    async handleOpen() {
        //TODO: @AlexGarciaPrada See this, navigator not defined
        // eslint-disable-next-line
        this.localStream = await navigator.mediaDevices.getUserMedia({
            video: true,
            audio: true,
        });
        this.localVideoRef.nativeElement.srcObject = this.localStream;

        this.peerConnection = new RTCPeerConnection(this.configuration);

        this.localStream
            .getTracks()
            .forEach((track) =>
                this.peerConnection.addTrack(track, this.localStream),
            );

        let counter = 0;
        this.peerConnection.ontrack = (event) => {
            if (counter < 2) {
                counter += 1;
                return;
            }
            const [stream] = event.streams;
            const userId = stream.id;

            if (!this.remoteStreams.has(userId)) {
                this.remoteStreams.set(userId, stream);

                const container = document.createElement("div");
                container.className = "user-media";
                container.id = `remote-user-${userId}`;

                const video = document.createElement("video");
                video.autoplay = true;
                video.playsInline = true;
                video.srcObject = stream;

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
            room_id: this.groupId,
        });
    }

    async handleMessage(data: MessageFormat) {
        if (data.type === "offer") {
            await this.peerConnection.setRemoteDescription(
                new RTCSessionDescription(data),
            );
            const answer = await this.peerConnection.createAnswer();
            await this.peerConnection.setLocalDescription(answer);
            this.websocketService.send({
                type: WebSocketMessageType.RTCAnswer,
                answer,
            });
        } else if (data.type === "candidate") {
            console.log(data);
            await this.peerConnection.addIceCandidate(
                new RTCIceCandidate(data.candidate),
            );
        }
    }

    //TODO: @AlexGarciaPrada This only stop sending but still captures
    toggleCamera() {
        if (!this.localStream) return;
        this.cameraOn = !this.cameraOn;
        this.localStream.getVideoTracks().forEach((track) => {
            track.enabled = this.cameraOn;
        });
        if (this.cameraOn) {
            console.log("Camera On");
        } else {
            console.log("Camera Off");
        }
    }

    toggleMic() {
        if (!this.localStream) return;
        this.micOn = !this.micOn;
        this.localStream.getAudioTracks().forEach((track) => {
            track.enabled = this.micOn;
        });
        if (this.micOn) {
            console.log("Voice On");
        } else {
            console.log("Voice Off");
        }
    }
}
