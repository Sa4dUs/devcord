import { Component, Input } from '@angular/core';
import { CdkDrag } from '@angular/cdk/drag-drop';

@Component({
  selector: 'bubble',
  templateUrl: './bubble.component.html',
  styleUrls: ['./bubble.component.scss'],
  standalone: true,
  imports: [CdkDrag],
})
export class BubbleComponent {
  @Input() boundarySelector = '';
}
