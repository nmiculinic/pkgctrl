package pacman

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPacmanProvider(t *testing.T) {
	p := &Provider{}
	pkgs, err := p.ListInstalledPackages(context.Background())
	assert.NoError(t, err)
	t.Log(pkgs)
}
