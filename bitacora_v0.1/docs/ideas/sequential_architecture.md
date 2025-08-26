IDEA:
# Templates de respuesta para disenar Arquitectura Secuencial 
Las arquitectura secuencia es aquella en la que creo crates que ejecuten funcionalidades especificas que de por si no dependen de otros crates.
Yo defino esta arquitectura como un transformer que peude procesar datos segun la necesidad de prioridad en secuencia pero se pueden combinar en paralelo y de manera trasnsversal.

# Templates de respuesta para disenar procesos secuenciales
Este concepto pretende analizar como podemos hacer (Por medio de prompts) que la AI comprenda de manera estandarizada como podemos hacer que cada que proceda entiendiendo el proceso anterior y el proximo contextualmente y de esa manera siempre tome desiciones para ejecutar acciones que siempre sean coherente la roadmap del proyecto.
Las prespuestas siempren tienen que presentar de manera practica y concreta la que accion que se realizo si es coherente con el roadmap especifico y general del TOPIC y el PROJECT.

Ejemplo practico para cualquier proyecto:
```
 <#1 process name> ↦ <#2 process name> ↦ <#3 process name> ↦ <#x process name + #x process name>
    ↘                   ↗
      <#x process name>
```

Ejemplo practico para el proyecto Bitacora:
```
 Project ↦ Topic ↦ Action
    ↘       ↗
      Spark

 Project ↦ Topic ↦ Action
             ↘       ↗
               Spark
```


