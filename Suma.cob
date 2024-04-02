       IDENTIFICATION DIVISION.
       PROGRAM-ID. SumaNumeros.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 Num1 PIC 9(5) VALUE ZERO.
       01 Num2 PIC 9(5) VALUE ZERO.
       01 Suma PIC 9(6) VALUE ZERO.
       01 InputBuffer PIC X(10).

       PROCEDURE DIVISION.
           DISPLAY "Calculadora de suma en COBOL".      
           DISPLAY "Ingrese el primer número:".
           ACCEPT InputBuffer.
           STRING InputBuffer DELIMITED BY SIZE INTO Num1.
           
           DISPLAY "Ingrese el segundo número:".
           ACCEPT InputBuffer.
           STRING InputBuffer DELIMITED BY SIZE INTO Num2.

           COMPUTE Suma = Num1 + Num2.
           DISPLAY "La suma de " Num1 " y " Num2 " es: " Suma.
           STOP RUN.
