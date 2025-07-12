import { Component } from '@angular/core';
import {  FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';


@Component({
  selector: 'app-register',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss']
})
export class RegisterComponent {
  registerForm: FormGroup; // Aquí guardamos el formulario y sus movidas

  constructor(private fb: FormBuilder, private http: HttpClient, private router: Router) {
    // Creamos el formulario:
    this.registerForm = this.fb.group({
      username: ['', Validators.required], // Campo obligatorio
      email: ['', [Validators.required, Validators.email]], 
      password: ['', Validators.required], 
      telephone: [''], //opcional
      prefix:['']//sobreentiendo opcional
    });
  }

  // Función que se lanza cuando pulsas el botón de "Registrarse"
  onSubmit(): void {
  if (this.registerForm.valid) {
    const { username, email, password, telephone } = this.registerForm.value;

    this.http.post<any>('http://lamoara.duckdns.org:6969/auth/register', {username,email,password,telephone}).subscribe({
      next: (data) => {
        // Hasta ahora no guardaba jeje. La sintaxis me la creo...la verdad
        localStorage.setItem('user', JSON.stringify({ username, email,password, telephone }));

        if (data.token) {
          localStorage.setItem('token', data.token);
        }

        console.log('Registro exitoso:', data);
        this.router.navigate(['/user']); //redirección al perfil, pudiera cambiarse pero por ahora se queda
      },
      error: (error) => {
        switch (error.status) {
          case 409:
            console.warn('Nombre de usuario o email ya están en uso.');
            break;
          case 500:
            console.error('Error interno del servidor. Llama al chamán del servidor.');
            break;
          default:
            console.error('Error desconocido:', error);
        }
      }
    });

    console.log('Datos enviados:', this.registerForm.value);
  } else {
    console.warn('Por la gloria del Imperio Mongol, rellena bien el formulario.');
  }
}

}
