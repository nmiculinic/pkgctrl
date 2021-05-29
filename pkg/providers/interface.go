package providers

import (
	"context"

	"github.com/nmiculinic/pkgctrl/pkg"
)

type Interface interface {
	ListInstalledPackages(ctx context.Context) ([]pkg.Package, error)
	InstallPackage(context.Context, pkg.Package) error
	UninstallPackage(context.Context, pkg.Package) error
}
