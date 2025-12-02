;;; day1.el --- Solving Day 1 of Advent Of Code 2025 -*- lexical-binding: t -*-
;;; Commentary:
;;; Code:

(require 'cl-lib)

(defun aoc2025/setup-test-buf ()
  "Read in a known test sequence for verification, used for AOC2025 Day 1."
  (goto-char (point-min))
  (insert "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82")
  (goto-char (point-min)))

(defun aoc2025/setup-predef-buf ()
  "Read in some preset test sequence for verification, used for AOC2025 Day 1."
  (goto-char (point-min))
  (insert "L150
L150
L150")
  (goto-char (point-min)))

(defun aoc2025/setup-buf ()
  "Read in the input for the buffer, used for AOC2025 Day 1."

  ;; read file into buffer
  (insert-file-contents "input" nil nil nil t)
  ;; set at begin
  (goto-char (point-min)))

(defun aoc2025/parse-update-rot (curr-rot)
  "Parse the buffer to create a new CURR-ROT, returning unaligned CURR-ROT."
  
  (save-excursion
    (let* ((curr-item (current-word))
	   ;; get direction
	   (dir (aref curr-item 0))
	   ;; and how much to move
	   (move (string-to-number (seq-drop curr-item 1))))
      (if (char-equal dir ?L)
	  ;; rot left
	  (setq curr-rot (- curr-rot move))
	;; rot right
	(setq curr-rot (+ curr-rot move))))
    curr-rot))

(message "at-zero: %d"
 (with-temp-buffer
   (aoc2025/setup-buf)
   ;; set position to mutate
   (let* ((curr-rot 50) (at-zero 0))
     ;; while not eof
     (while (not (eobp))
       (setq curr-rot (aoc2025/parse-update-rot curr-rot))
       ;; readjust dial position each pass
       (while (< curr-rot 0)
	 (setq curr-rot (+ curr-rot 100)))
       (while (> curr-rot 99)
	 (setq curr-rot (- curr-rot 100)))
       ;; if at zero, inc
       (when (eql curr-rot 0)
	 (setq at-zero (1+ at-zero)))
       ;; move forward one line
       (forward-line 1))
     ;; return
     at-zero)))

(message "click-zero: %d"
	 (with-temp-buffer
	   (aoc2025/setup-buf)
	   ;; set mutating vars
	   (let* ((curr-rot 50) (click-zero 0))
	     (while (not (eobp))
	       (let* ((was-zero (eql curr-rot 0)))
		 (setq curr-rot (aoc2025/parse-update-rot curr-rot))
		 (let* ((at-zero (eql curr-rot 0))
			(lower (cl-minusp curr-rot))
			(higher (> curr-rot 99)))
		   ;; using the conds determined above, do action below
		   (cond ((and at-zero (not was-zero)) (setq click-zero (1+ click-zero)))
			 ;; on higher, get int div and simply set to modulo
			 (higher (setq click-zero (+ (/ curr-rot 100) click-zero)
				       curr-rot (% curr-rot 100)))
			 ;; on lower, negate the int div and add 1 full rotation to the dial if the prev wasnt zero
			 (lower
			  (setq click-zero (+ (/ (abs curr-rot) 100) click-zero (if was-zero '0 '1))
				curr-rot (+ 100 (% curr-rot 100)))
			  ;; correct an off by one err when doing the rem
			  ;; tldr: when at a negative multiple of 100 (-100, -200, -300, ...)
			  ;; the curr rot becomes 0 which is added to 100. this is because of how
			  ;; the curr rot is recalcuated, taking advantage of how the rem negative produces a
			  ;; negative number, which inverts 100 to get the correct number. this breaks down
			  ;; when the rem is zero, as it doesnt invert 100 properly, so it must be set to zero.
			  (when (eql curr-rot 100)
			    (setq curr-rot 0)))))
		 (forward-line 1)))
	     click-zero)))
;;; day1.el ends here
