package qqml

type ErrSrcPathNotSpecified struct{}

func (e ErrSrcPathNotSpecified) Error() string {
	return "The srcPath value has not been specified!"
}

type ErrNoSuchSrcType struct{}

func (e ErrNoSuchSrcType) Error() string {
	return "The specified SrcType value is out of bounds"
}

type ErrOutPathNotSpecified struct{}

func (e ErrOutPathNotSpecified) Error() string {
	return "The outPath value has not been specified!"
}
