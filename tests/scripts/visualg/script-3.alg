Algoritmo "semnome"

Var

I,TABUADA,D:INTEIRO
OP:CARACTER


Inicio
  I<-0


  ENQUANTO (OP <> "S") FACA

      REPITA
      ESCREVAL("QUAL TABUADA DESEJA OBTER:")
      LEIA(D)

         I<-I+1
         TABUADA<-I*D
         ESCREVAL(D,"X" ,I, "= ",TABUADA)

      ATE I < 9
      
  ESCREVAL("DESEJA OUTRA TABUADA S/N")
  LEIA(OP)
  FIMENQUANTO
      



Fimalgoritmo