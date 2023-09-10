package locate

import "qpm/internal"

type ErrNotInitialised struct{}

func (e *ErrNotInitialised) Error() string {
	return "QPM is not initialised, please run qpm init"
}

func findSrcFile(name string) ([]string, error) {
	res, err := internal.IsInitialised()
	if err != nil {
		return nil, err
	}
	if !res {
		return nil, &ErrNotInitialised{}
	}

	return []string{}, nil
}
