import { Component, ElementRef, ViewChild } from "@angular/core";

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

    private localStream!: MediaStream;
    private peerConnection!: RTCPeerConnection;
    private ws!: WebSocket;
    private remoteStreams = new Map<string, MediaStream>();

    private configuration: RTCConfiguration = {
        iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
    };

    async startCallPaz(): Promise<void> {
        const token_pez =
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTY2NDI5ODksInVzZXJfaWQiOiI2NzU4MmQ2Zi1jNmFlLTQzYTAtYTQ1My03ZThjODY5NDNmYjAifQ.8zbULF7TkIVZFBlKsHZaAPkHtQBycyGY__XnjA2V1JQ";
        const token_paz =
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTY2NDI5MzksInVzZXJfaWQiOiI3NTVlMjA1Mi02MzE1LTQ5MTEtOGRmYS00YWVjNzRmYmY2OTEifQ.RkJ2tfLIkfCAIcEoeEUis1yD3qlC_oXM6L7Zps1k5Wc";


        this.ws = new WebSocket("ws://192.168.1.145:3002", [token_paz]);

        this.ws.addEventListener("open", async () => {
            this.localStream = await navigator.mediaDevices.getUserMedia({
                video: false,
                audio: true,
            });
            //this.localVideoRef.nativeElement.srcObject = this.localStream;

            console.log("Total tracks: ", this.localStream.getTracks().length);

            this.peerConnection = new RTCPeerConnection(this.configuration);

            this.localStream.getTracks().forEach((track, index) => {
                console.log(
                    `Track #${index}: kind=${track.kind}, id=${track.id}`,
                );
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
                            .catch((err) =>
                                console.warn("Video play error:", err),
                            );
                    };

                    const label = document.createElement("div");
                    label.textContent = `Usuario: ${userId}`;

                    container.appendChild(label);
                    container.appendChild(video);
                    this.remoteMediaContainerRef.nativeElement.appendChild(
                        container,
                    );
                } else {
                    console.log(
                        "Stream adicional para usuario existente:",
                        userId,
                    );
                }
            };

            this.peerConnection.onicecandidate = (event) => {
                if (event.candidate) {
                    this.ws.send(
                        JSON.stringify({
                            type: "rtccandidate",
                            candidate: event.candidate,
                        }),
                    );
                }
            };

            this.ws.send(
                JSON.stringify({ type: "connecttoroom", room_id: "A" }),
            );
        });

        this.ws.addEventListener("message", async (message) => {
            const data = JSON.parse(message.data);

            if (data.type === "offer") {
                await this.peerConnection.setRemoteDescription(
                    new RTCSessionDescription(data),
                );
                const answer = await this.peerConnection.createAnswer();
                await this.peerConnection.setLocalDescription(answer);
                this.ws.send(JSON.stringify({ type: "rtcanswer", answer }));
            } else if (data.type === "answer") {
                //Maybe is neccesary to handle
            } else if (data.type === "candidate") {
                await this.peerConnection.addIceCandidate(
                    new RTCIceCandidate(data.candidate),
                );
            }
        });
    }
     async startCallPez(): Promise<void> {
        const token_pez =
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTY2NDI5ODksInVzZXJfaWQiOiI2NzU4MmQ2Zi1jNmFlLTQzYTAtYTQ1My03ZThjODY5NDNmYjAifQ.8zbULF7TkIVZFBlKsHZaAPkHtQBycyGY__XnjA2V1JQ";
        const token_paz =
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTY2NDI5MzksInVzZXJfaWQiOiI3NTVlMjA1Mi02MzE1LTQ5MTEtOGRmYS00YWVjNzRmYmY2OTEifQ.RkJ2tfLIkfCAIcEoeEUis1yD3qlC_oXM6L7Zps1k5Wc";


        this.ws = new WebSocket("ws://192.168.1.145:3002", [token_pez]);

        this.ws.addEventListener("open", async () => {
            this.localStream = await navigator.mediaDevices.getUserMedia({
                video: false,
                audio: true,
            });
            //this.localVideoRef.nativeElement.srcObject = this.localStream;

            console.log("Total tracks: ", this.localStream.getTracks().length);

            this.peerConnection = new RTCPeerConnection(this.configuration);

            this.localStream.getTracks().forEach((track, index) => {
                console.log(
                    `Track #${index}: kind=${track.kind}, id=${track.id}`,
                );
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
                            .catch((err) =>
                                console.warn("Video play error:", err),
                            );
                    };

                    const label = document.createElement("div");
                    label.textContent = `Usuario: ${userId}`;

                    container.appendChild(label);
                    container.appendChild(video);
                    this.remoteMediaContainerRef.nativeElement.appendChild(
                        container,
                    );
                } else {
                    console.log(
                        "Stream adicional para usuario existente:",
                        userId,
                    );
                }
            };

            this.peerConnection.onicecandidate = (event) => {
                if (event.candidate) {
                    this.ws.send(
                        JSON.stringify({
                            type: "rtccandidate",
                            candidate: event.candidate,
                        }),
                    );
                }
            };

            this.ws.send(
                JSON.stringify({ type: "connecttoroom", room_id: "A" }),
            );
        });

        this.ws.addEventListener("message", async (message) => {
            const data = JSON.parse(message.data);

            if (data.type === "offer") {
                await this.peerConnection.setRemoteDescription(
                    new RTCSessionDescription(data),
                );
                const answer = await this.peerConnection.createAnswer();
                await this.peerConnection.setLocalDescription(answer);
                this.ws.send(JSON.stringify({ type: "rtcanswer", answer }));
            } else if (data.type === "answer") {
                //Maybe is neccesary to handle
            } else if (data.type === "candidate") {
                await this.peerConnection.addIceCandidate(
                    new RTCIceCandidate(data.candidate),
                );
            }
        });
    }

    endCall(): void {
        if (this.peerConnection) this.peerConnection.close();
        if (this.localStream)
            this.localStream.getTracks().forEach((track) => track.stop());
        this.localVideoRef.nativeElement.srcObject = null;

        for (const [id] of this.remoteStreams) {
            const el = document.getElementById(`remote-user-${id}`);
            if (el) el.remove();
        }

        this.remoteStreams.clear();
        if (this.ws) this.ws.close();
    }
}
