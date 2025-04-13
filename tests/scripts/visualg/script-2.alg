Algoritmo "Exercicio 1"

Var

i: inteiro //valor auxiliar que cuidara da sequencia de numeros na fun��o para
j: inteiro //valor auxiliar que cuidara dos numeros negativos
k: inteiro //valor auxiliar que cuidara dos numeros positivos
l: inteiro //valor auxiliar que cuidara dos numeros igauis a zeros

NPositivos: inteiro //quantidade final de numeros positivos
NNegativos: inteiro //quantidade final de numeros negativos
NZeros: inteiro //quantidade final de numeros iguais a zero
Numeros: inteiro //quantidade total ded numeros

Inicio

Numeros<- randi(500) //o valor do numero sera aleatorio de 0 ate 500

para i de Numeros - 500 ate Numeros - 1 faca //para executar a função ele ira subtrair o valor por 499
//para termos 500 valores, alguns negativos e outros positivos e um, sempre, igual a zero.

     j<-i //definimos os valores de cada funçao auxiliar para usarmos elas nas funções de enquanto
     k<-i
     l<-i
     
     enquanto j < 0 faca //aqui ele ira verificara se o j, valor auxiliar dos numeros negativos,
              //possui um valor menor do que zero, ou seja, negativo.
              //se seu valor for negativo ele ira adicionar um numero na conta dos numeros negativos
                NNegativos<- NNegativos+1
                j<-0
     fimenquanto
     
     enquanto k > 0 faca//aqui a mesma coisa, porem checando se o numero positivo
                NPositivos<- NPositivos+1
                k<-0
     fimenquanto
     
     enquanto l = 0 faca//e aqui para checar se o numero igual a zero.
                NZeros<- NZeros+1
                l<-1
     fimenquanto
     
fimpara

escreval("Numeros positivos: ", NPositivos)
escreval("Numeros negativos: ", NNegativos)
escreval("Numeros iguais a zero: ", NZeros)
escreval("Numeros totais: ", NPositivos + NNegativos + NZeros)
//aqui ele so ira escrever a quantidade de numeros positivos, negativos e iguais a zero.

Fimalgoritmo