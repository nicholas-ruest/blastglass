package blastglass
default allow := false
allow if { input.role == "rehearsal-operator"; input.blast_radius <= 80 }
