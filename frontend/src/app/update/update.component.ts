import { Component, OnInit, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-update',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './update.component.html',
  styleUrls: ['./update.component.scss']
})



export class UpdateUsernameComponent implements OnInit {
  newUsername: string = '';
  success: string | null = null;
  error: string | null = null;
  loading: boolean = false;

  constructor(
    private http: HttpClient,
    @Inject(PLATFORM_ID) private platformId: Object
  ) {}

  ngOnInit(): void {}

  updateUsername(): void {
    if (!isPlatformBrowser(this.platformId)) {
      console.log('No estamos en un navegador, no se puede continuar');
      return;
    }

    const token = localStorage.getItem('token');

    if (!token) {
      this.error = 'No estás logueado payaso.';
      console.error('Error: No token');
      return;
    }

    if (!this.newUsername.trim()) {
      this.error = 'Hasta los fantasmas tienen nombre, que tú no lo tengas es lamentable.';
      console.error('Error: nuevo nombre vacío');
      return;
    }

    this.loading = true;
    this.error = null;
    this.success = null;

    const body = {
      query: {
        Username: this.newUsername.trim()
      }
    };

    this.http.post('http://lamoara.duckdns.org:6969/api/user/update', body, {
      headers: { Authorization: `Bearer ${token}` }
    }).subscribe({
      next: () => {
        console.log('Usuario actualizado correctamente');
        this.success = `Tu nuevo nombre de usuario es "${this.newUsername}".`;
        this.newUsername = '';
        this.loading = false;
      },
      error: (error) => {
        console.log('Error en la petición:', error);
        switch (error.status) { //en el documento no venía ningún control de errores pero estos son los típicos por lo que llevo hecho
          case 400:
            console.warn('El nuevo nombre no es válido o ya está cogido.');
            break;
          case 401:
            console.error('No autorizado. Revisa tu token.');
            break;
          default:
            console.error('Error desconocido');
        }
        this.error = 'No se pudo actualizar el nombre de usuario.';
        this.loading = false;
      }
    });
  }
}
