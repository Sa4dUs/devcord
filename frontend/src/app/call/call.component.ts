import { Component, ElementRef, ViewChild, AfterViewInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { WebcamModule, WebcamImage, WebcamInitError, WebcamUtil } from 'ngx-webcam';
import { Subject, Observable, flatMap } from 'rxjs';
import { CameraButton } from "./cameraButton/cameraButton.component";

@Component({
  selector: 'call',
  standalone: true,
  imports: [CommonModule, WebcamModule, CameraButton],
  templateUrl: './call.component.html',
  styleUrls: ['./call.component.scss']
})
export class Call implements AfterViewInit {
  @ViewChild('canvas') canvas!: ElementRef<HTMLCanvasElement>;

  public showWebcam = true;
  public multipleWebcamsAvailable = false;
  public height = 400;
  public width = 600;
  public trigger: Subject<void> = new Subject<void>();
  public webcamImage?: WebcamImage;
  public errors: WebcamInitError[] = [];
  public autoSaveAfterCapture = false;

  public videoOptions: MediaTrackConstraints = {
    width: { ideal: this.width },
    height: { ideal: this.height },
    facingMode: 'user'
  };

  ngAfterViewInit(): void {
    WebcamUtil.getAvailableVideoInputs()
      .then((mediaDevices: MediaDeviceInfo[]) => {
        this.multipleWebcamsAvailable = mediaDevices.length > 1;
      })
      .catch(err => console.error("enumerateDevices error:", err));
  }

  public triggerObservable(): Observable<void> {
    return this.trigger.asObservable();
  }

  public toggleWebcam(): void {
    this.showWebcam = !this.showWebcam;
  }

  public handleInitError(error: WebcamInitError): void {
    this.errors.push(error);
  }

  public handleImage(webcamImage: WebcamImage): void {
    this.webcamImage = webcamImage;
    const ctx = this.canvas.nativeElement.getContext('2d');
    if (ctx) {
      const img = new Image();
      img.onload = () => {
        this.canvas.nativeElement.width = img.width;
        this.canvas.nativeElement.height = img.height;
        ctx.drawImage(img, 0, 0);

        if (this.autoSaveAfterCapture) {
          this.downloadImage();
        }
      };
      img.src = webcamImage.imageAsDataUrl;
    }
  }

  public capture(): void {
    this.trigger.next();
  }

  private dataURLToBlob(dataURL: string): Blob {
    const [header, data] = dataURL.split(',');
    const mimeMatch = header.match(/:(.*?);/);
    const mime = mimeMatch ? mimeMatch[1] : 'image/png';
    const binary = atob(data);
    const len = binary.length;
    const u8 = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
      u8[i] = binary.charCodeAt(i);
    }
    return new Blob([u8], { type: mime });
  }

  public downloadImage(): void {
    if (!this.canvas || !this.canvas.nativeElement) { return; }

    const dataURL = this.canvas.nativeElement.toDataURL('image/png');

    const blob = this.dataURLToBlob(dataURL);
    const url = URL.createObjectURL(blob);

    const a = document.createElement('a');
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-'); // nombre seguro
    a.href = url;
    a.download = `foto-${timestamp}.png`;
    a.click();

    URL.revokeObjectURL(url);
  }
}
