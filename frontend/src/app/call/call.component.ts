import {
    Component,
    ElementRef,
    ViewChild,
    AfterViewInit,
    OnDestroy,
} from "@angular/core";
import { CommonModule } from "@angular/common";

@Component({
    selector: "call",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./call.component.html",
    styleUrls: ["./call.component.scss"],
})
export class Call implements AfterViewInit, OnDestroy {
    @ViewChild("localVideo") localVideo!: ElementRef<HTMLVideoElement>;
    @ViewChild("remoteVideo") remoteVideo!: ElementRef<HTMLVideoElement>;

    private localStream!: MediaStream;
    private localPeer!: RTCPeerConnection;
    private remotePeer!: RTCPeerConnection;

    private isCameraOn = true;
    private isAudioOn = true;

    get cameraOn(): boolean {
        return this.isCameraOn;
    }

    get audioOn(): boolean {
        return this.isAudioOn;
    }

    async ngAfterViewInit() {
        await this.startLocalStream();
        await this.setupPeerConnection();
    }

    private async startLocalStream() {
        this.localStream = await navigator.mediaDevices.getUserMedia({
            video: true,
            audio: true,
        });
        this.localVideo.nativeElement.srcObject = this.localStream;
    }

    private async setupPeerConnection() {
        this.localPeer = new RTCPeerConnection();
        this.remotePeer = new RTCPeerConnection();

        this.localStream
            .getTracks()
            .forEach((track) =>
                this.localPeer.addTrack(track, this.localStream),
            );

        this.remotePeer.ontrack = (event) => {
            this.remoteVideo.nativeElement.srcObject = event.streams[0];
        };

        this.localPeer.onicecandidate = (event) => {
            if (event.candidate) {
                this.remotePeer.addIceCandidate(event.candidate);
            }
        };
        this.remotePeer.onicecandidate = (event) => {
            if (event.candidate) {
                this.localPeer.addIceCandidate(event.candidate);
            }
        };

        const offer = await this.localPeer.createOffer();
        await this.localPeer.setLocalDescription(offer);
        await this.remotePeer.setRemoteDescription(offer);

        const answer = await this.remotePeer.createAnswer();
        await this.remotePeer.setLocalDescription(answer);
        await this.localPeer.setRemoteDescription(answer);
    }

    ngOnDestroy() {
        this.localPeer?.close();
        this.remotePeer?.close();
        this.localStream?.getTracks().forEach((t) => t.stop());
    }

    toggleAudio() {
        this.isAudioOn = !this.isAudioOn;
        this.localStream
            ?.getAudioTracks()
            .forEach((track) => (track.enabled = this.isAudioOn));
    }

    async toggleCamera() {
        if (this.isCameraOn) {
            this.localStream.getVideoTracks().forEach((track) => track.stop());
            this.isCameraOn = false;
        } else {
            const newStream = await navigator.mediaDevices.getUserMedia({
                video: true,
            });
            const videoTrack = newStream.getVideoTracks()[0];

            const sender = this.localPeer
                .getSenders()
                .find((s) => s.track?.kind === "video");
            if (sender) {
                sender.replaceTrack(videoTrack);
            }

            const oldTrack = this.localStream.getVideoTracks()[0];
            if (oldTrack) this.localStream.removeTrack(oldTrack);
            this.localStream.addTrack(videoTrack);

            this.localVideo.nativeElement.srcObject = this.localStream;
            this.isCameraOn = true;
        }
    }
}
