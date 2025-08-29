# SPEC_ALIASES.md · Especificación de alias BitaFlow

## Formato (humano)
```
<NAMESPACE>-<KIND>-<TOPIC>(-<SUBTOPIC>)?-v<MAJOR>(+<TAG>)*
```

**Campos**
- NAMESPACE: `BITA`
- KIND: `TPL|DOC|ADR|SNAP|MAP`
- TOPIC: `[A-Z0-9]+(_[A-Z0-9]+)*`
- SUBTOPIC (opcional)
- Versión: `v` + dígitos
- TAG (opcional): `+ES`, `+API`, `+EXEC`, etc.

**Regex (CI):**
```
^BITA-(TPL|DOC|ADR|SNAP|MAP)-[A-Z0-9]+(?:_[A-Z0-9]+)*(?:-[A-Z0-9]+(?:_[A-Z0-9]+)*)?-v[0-9]+(?:\+[A-Z0-9]+)*$
```

**EBNF**
```
ALIAS        := NAMESPACE "-" KIND "-" TOPIC ( "-" SUBTOPIC )? "-" VERSION ( TAG )* ;
NAMESPACE    := "BITA" ;
KIND         := "TPL" | "DOC" | "ADR" | "SNAP" | "MAP" ;
TOPIC        := UPPER_WORD ( "_" UPPER_WORD )* ;
SUBTOPIC     := UPPER_WORD ( "_" UPPER_WORD )* ;
VERSION      := "v" DIGITS ;
TAG          := "+" UPPER_WORD ;
UPPER_WORD   := [A-Z0-9]+ ;
DIGITS       := [0-9]+ ;
```
