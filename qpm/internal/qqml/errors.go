//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

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
