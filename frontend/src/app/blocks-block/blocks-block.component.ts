import { Component, OnInit, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-blocks-block',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './blocks-block.component.html',
  styleUrls: ['./blocks-block.component.scss']
})


export class BlocksBlockComponent implements OnInit {
  toUserUsername: string = '';  // <-- nombre coherente con el backend y Angular
  success: string | null = null;
  error: string | null = null;
  loading: boolean = false;

  constructor(
    private http: HttpClient,
    @Inject(PLATFORM_ID) private platformId: Object
  ) {}

  ngOnInit(): void {}

blockUser(): void {

  if (!isPlatformBrowser(this.platformId)) {
    console.log('No estamos en navegador, no se puede continuar');
    return;
  }

  const token = localStorage.getItem('token');

  if (!token) {
    this.error = 'No estás logueado.';
    console.error('Error: No token');
    return;
  }

  if (!this.toUserUsername.trim()) {
    this.error = 'El nombre de usuario no puede estar vacío.';
    console.error('Error: nombre de usuario vacío');
    return;
  }

  this.loading = true;
  this.error = null;
  this.success = null;

  const body = {
    to_user_username: this.toUserUsername.trim()
  };

  this.http.post('http://lamoara.duckdns.org:6969/api/user/blocks/block', body, {
    headers: { Authorization: `Bearer ${token}` }
  }).subscribe({
    next: () => {
      console.log('Respuesta exitosa del backend');
      this.success = `Usuario ${this.toUserUsername} bloqueado, a este paso te quedarás sin amigos...`;
      this.toUserUsername = '';
      this.loading = false;
    },
    error: (error) => {
      console.log('Error en la petición:', error);
      switch(error.status){
        case 404:
          console.warn('No puedes bloquear fantasmas');
          break;
        case 409:
          console.error('Ya le habías bloqueado antes');
          break;
        default:
          console.error('Error desconocido');
      }
      this.loading = false;
    }
  });
}

}
