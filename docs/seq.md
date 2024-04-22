# Seq

(drop seq n) ; drop first n elements

(take seq n) ; return (and remove) first n elements

(take seq) ; return (and remove) first element, like JavaScript's Array.shift()

(put seq el) ; puts the element el at the beginning of the seq, like JavaScript's Array.unshift()

(push seq el) ; puts the element el at the end of the seq

(pop seq) ; return (and remove) last element

(get seq idx) ; get reference to the element at index idx

(put seq el idx) ; put the element el at the specific index idx

(remove seq idx)

(join seq separator) ; find better name

(reduce seq (-> [acc, x] (+ acc x)))

(fold seq fn)or
(fold fn seq) ; fn is the receiver

(map seq fn) or 
(map fn seq) ; fn is the receiver this


