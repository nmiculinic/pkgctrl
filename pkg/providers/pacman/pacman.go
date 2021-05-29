package pacman

import (
	"context"
	"fmt"
	"os/exec"
	"strings"

	"github.com/nmiculinic/pkgctrl/pkg"
	"github.com/nmiculinic/pkgctrl/pkg/providers"
)

type Provider struct{}

var _ providers.Interface = &Provider{}

func (p Provider) ListInstalledPackages(ctx context.Context) ([]pkg.Package, error) {
	out, err := exec.CommandContext(ctx, "pacman", "-Qi").CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("cannot execute pacman: %s", err)
	}
	var pkgs []pkg.Package
	current := pkg.Package{}

	currentField := ""
	for _, line := range strings.Split(string(out), "\n") {
		line := strings.Trim(line, " ")
		if line == "" {
			if current.Name != "" {
				pkgs = append(pkgs, current)
			}
			current = pkg.Package{}
			continue
		}
		elements := strings.SplitN(line, ":", 2)
		values := ""
		switch len(elements) {
		case 1:
			values = strings.Trim(elements[0], " ")
		case 2:
			currentField = strings.Trim(elements[0], " ")
			values = strings.Trim(elements[1], " ")
		default:
			return nil, fmt.Errorf("found multiple colons in: %s", line)
		}
		switch currentField {
		case "Name":
			current.Name = values
		case "Version":
		case "Description":
		case "Architecture":
		case "URL":
			current.URL = values
		case "Licenses":
		case "Groups":
		case "Provides":
		case "Depends On":
		case "Optional Deps":
		case "Required By":
		case "Optional For":
		case "Conflicts With":
		case "Replaces":
		case "Installed Size":
		case "Packager":
		case "Build Date":
		case "Install Date":
		case "Install Reason":
		case "Install Script":
		case "Validated By":
		default:
			return nil, fmt.Errorf("unknown package property: %s", currentField)
		}
	}
	return pkgs, nil
}

func (p Provider) InstallPackage(ctx context.Context, p2 pkg.Package) error {
	panic("implement me")
}

func (p Provider) UninstallPackage(ctx context.Context, p2 pkg.Package) error {
	panic("implement me")
}
