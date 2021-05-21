param qtTarefas;
param tempoDisponivel;

set X := {1..qtTarefas};

param Tempo {X} >= 0;
param Pontos {X} >= 0;

var devoFazerTX {X} >= 0, <= 1;

maximize MaxPts: sum {i in X} devoFazerTX[i] * Pontos[i];

subj to TempoDisp: 
	sum {i in X} devoFazerTX[i] * Tempo[i] <= tempoDisponivel;