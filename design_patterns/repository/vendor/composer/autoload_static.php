<?php

// autoload_static.php @generated by Composer

namespace Composer\Autoload;

class ComposerStaticInit432110ff2f864a0b63a3e8f09bcb7498
{
    public static $prefixLengthsPsr4 = array (
        'A' => 
        array (
            'App\\' => 4,
        ),
    );

    public static $prefixDirsPsr4 = array (
        'App\\' => 
        array (
            0 => __DIR__ . '/../..' . '/app',
        ),
    );

    public static $classMap = array (
        'Composer\\InstalledVersions' => __DIR__ . '/..' . '/composer/InstalledVersions.php',
    );

    public static function getInitializer(ClassLoader $loader)
    {
        return \Closure::bind(function () use ($loader) {
            $loader->prefixLengthsPsr4 = ComposerStaticInit432110ff2f864a0b63a3e8f09bcb7498::$prefixLengthsPsr4;
            $loader->prefixDirsPsr4 = ComposerStaticInit432110ff2f864a0b63a3e8f09bcb7498::$prefixDirsPsr4;
            $loader->classMap = ComposerStaticInit432110ff2f864a0b63a3e8f09bcb7498::$classMap;

        }, null, ClassLoader::class);
    }
}