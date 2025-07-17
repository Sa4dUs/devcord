import { Component, OnInit, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-blocks-unblock',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './blocks-unblock.component.html',
  styleUrls: ['./blocks-unblock.component.scss']
})


export class BlocksUnblockComponent implements OnInit {
  toUserUsername: string = '';  // lo de los nombres es un lio histórico
  success: string | null = null;
  error: string | null = null;
  loading: boolean = false;

  constructor(
    private http: HttpClient,
    @Inject(PLATFORM_ID) private platformId: Object
  ) {}

  ngOnInit(): void {}

unblockUser(): void {

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

  this.http.post('http://lamoara.duckdns.org:6969/api/user/blocks/unblock', body, {
    headers: { Authorization: `Bearer ${token}` }
  }).subscribe({
    next: () => {
      console.log('Respuesta exitosa del backend');
      this.success = `Usuario ${this.toUserUsername} desbloqueado, enhorabuena te has hecho el interesante un rato...`;
      this.toUserUsername = '';
      this.loading = false;
    },
    error: (error) => {
      console.log('Error en la petición:', error);
      switch(error.status){
        case 404:
          console.warn('Seguirás solo un rato más...');
          break;
        default:
          console.error('Error desconocido');
      }
      this.loading = false;
    }
  });
}

}
