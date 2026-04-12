; Replaces FFI implementation using native JS hooks mapped to faiss

;Create a new atom-tagged vectorspace
(= (new-atom-vectorspace $space $dimension)
   (bind! $space (new-state ((js-call (faiss.create $dimension)) $dimension))))

;Custom vector:
(= (add-atom-vector $space $atom $vector)
   (let* (($vdb_and_dim (get-state $space))
          ($vdb (car-atom $vdb_and_dim)))
         (js-call (faiss.add $vdb $atom $vector))))

;Structural Random Indexing
(= (add-atom-SRI $space $atom)
   (let* (($vdb_and_dim (get-state $space))
          ($vdb (car-atom $vdb_and_dim))
          ($dim (car-atom (cdr-atom $vdb_and_dim))))
         (js-call (faiss.add $vdb $atom (embed $atom $dim)))))

;Match in the space by similarity
(= (match-k $k $space $query)
   (let* (($vdb_and_dim (get-state $space))
          ($vdb (car-atom $vdb_and_dim)))
         (js-call (faiss.search $vdb $query $k))))

;Match using structural random embedding:
(= (match-SRI $k $space $query)
   (let* (($vdb_and_dim (get-state $space))
          ($vdb (car-atom $vdb_and_dim))
          ($dim (car-atom (cdr-atom $vdb_and_dim))))
         (js-call (faiss.search $vdb (embed $query $dim) $k))))

;Helper to extract atom from closest-match results
(= (atom-of $x) (car-atom $x))

;Embed helper mapping to JS logic
(= (embed $atom $dim)
   (js-call (faiss.embed $atom $dim)))
