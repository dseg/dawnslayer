((rust-mode . ((eval . (progn
                        (c-set-style "BSD")
                        (setq-default indent-tabs-mode nil)
                        (setq-default tab-width 4)
                        (setq rust-indent-offset 4)
			(setq rust-format-on-save t)
                        )))))
