import { Component } from '@angular/core';
import {  FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClient } from '@angular/common/http';


// Decorador mágico que le dice a Angular: "Ey, esto es un componente"
//Stalone seguirá siendo un misterio 
@Component({
  selector: 'app-register',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss']
})
export class RegisterComponent {
  registerForm: FormGroup; // Aquí guardamos el formulario y sus movidas

  constructor(private fb: FormBuilder, private http: HttpClient) {
    // Creamos el formulario:
    this.registerForm = this.fb.group({
      username: ['', Validators.required], // Campo obligatorio
      email: ['', [Validators.required, Validators.email]], 
      password: ['', Validators.required], 
      telephone: [''] //opcional
    });
  }

  // Función que se lanza cuando pulsas el botón de "Registrarse"
  onSubmit(): void {
    if (this.registerForm.valid) {
      const { username, email, password, telephone } = this.registerForm.value;
      this.http.post('http://localhost:3001', { username, password }).subscribe({ //esta línea es la que enlaza con el backend
        next: (data) => console.log('Registro exitoso:', data),
        error: (error) => console.warn('Gran Calamidad ocurrida:', error),
      });
      console.log('Datos del formulario:', this.registerForm.value);
    } else {
      console.warn('Por la gloria del Imperio Mongol, rellena el formulario!');
    }
  }
}
