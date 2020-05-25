from setuptools import setup, find_packages

def build_native(spec):
    # Step 1: build the rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    # Step 2: add a cffi module based on the dylib we built
    #
    # We use lambdas here for dylib and header_filename so that those are
    # only called after the external build finished.
    spec.add_cffi_module(
        module_path='excel_to_pydict._native',
        dylib=lambda: build.find_dylib('excel_to_pydict', in_path='target/release'),
        header_filename=lambda: build.find_header('excel_to_pydict.h', in_path='target'),
        rtld_flags=['NOW', 'NODELETE']
    )


setup(
    name='excel_to_pydict',
    version='0.0.1',
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    install_requires=[
        'milksnake',
    ],
    milksnake_tasks=[
        build_native,
    ]
)
